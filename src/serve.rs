use std::{collections::BTreeSet, env::current_dir};

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

use crate::{consensus::MessageIdentifier, system::FullSystemStateLocked};

// User -> Node (request to broadcast message)
#[derive(Debug, Deserialize)]
pub struct BroadcastRequestMessage {
    pub message: String,
}

// Node -> Node (broadcast)
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BroadcastMessage {
    pub id: MessageIdentifier,
    pub message: String,
}

pub async fn broadcast_message(
    State(state): State<FullSystemStateLocked>,
    Json(payload): Json<BroadcastRequestMessage>,
) -> StatusCode {
    info!("[broadcast request received]");

    // -- begin critical section
    let mut lock = state.write().expect("RW lock poisoned");
    let sender = lock.consensus.node_id;
    let deps = {
        let mut received = lock.consensus.received;
        received[sender] = lock.consensus.send_seq;
        received
    };
    lock.consensus.send_seq += 1;
    drop(lock); // 🫡
    // -- end critical section

    let broadcast_message = BroadcastMessage {
        id: MessageIdentifier { sender, deps },
        message: payload.message,
    };

    perform_broadcast(&broadcast_message, sender);

    StatusCode::CREATED
}

pub async fn receive_message(
    State(state): State<FullSystemStateLocked>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let mut lock = state.write().expect("RW lock poisoned");
    info!(
        "[broadcast message received] {} --> {}",
        payload.id.sender, lock.consensus.node_id
    );

    if lock.consensus.relayed.contains(&payload.id) {
        return StatusCode::CREATED;
    }
    lock.consensus.relayed.insert(payload.id.clone());
    lock.consensus.buffer.insert(payload.clone());

    loop {
        let mut try_again = false;
        let mut remove_indices = vec![];

        for (index, buffer) in lock.consensus.buffer.clone().iter().enumerate() {
            // Check if buffer is eligible for delivery
            let mut buffer_qualifies = (0..5)
                .into_iter()
                .all(|i| buffer.id.deps[i] <= lock.consensus.received[i]);

            // If it is, deliver the message
            if buffer_qualifies {
                lock.applicaton.messages.push(buffer.message.clone());
                lock.consensus.received[buffer.id.sender] += 1;
                remove_indices.push(index);
                try_again = true;
            }
        }

        let mut new_buffer: BTreeSet<_> = Default::default();
        for (index, buffer) in lock.consensus.buffer.clone().into_iter().enumerate() {
            if !remove_indices.iter().any(|&r| r == index) {
                new_buffer.insert(buffer);
            }
        }

        lock.consensus.buffer = new_buffer;

        if !try_again {
            break;
        }
    }

    info!("application state {:?}", lock.applicaton);
    perform_broadcast(&payload, lock.consensus.node_id);
    StatusCode::CREATED
}

pub fn perform_broadcast(broadcast_message: &BroadcastMessage, sender: usize) {
    for node_id in [sender, (sender + 1) % 5, (sender + 2) % 5] {
        let client = reqwest::Client::new();
        let broadcast_message = broadcast_message.clone();
        tokio::spawn(async move {
            match client
                .post(format!("http://0.0.0.0:{}/receive", 3000 + node_id))
                .json(&broadcast_message)
                .send()
                .await
            {
                Ok(response) => {
                    match response.error_for_status() {
                        Ok(_) => {
                            tracing::info!("[broadcast] {} ---> {}", sender, node_id);
                        }
                        Err(err) => {
                            tracing::error!("[broadcast failed] {:?}", err);
                        }
                    };
                }
                Err(err) => {
                    tracing::error!("[broadcast failed] {:?}", err);
                }
            };
        });
    }
}
