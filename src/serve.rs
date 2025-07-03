use std::{collections::BTreeSet, env::current_dir};

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

use crate::{app, consensus::MessageIdentifier, system::FullSystemStateLocked};

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
    let mut lock = state.write().expect("lock poisoned");
    let sender = lock.consensus.node_id;
    let deps = {
        let mut received = lock.consensus.delivered;
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
    info!("[broadcast received]");

    // -- begin critical section
    let mut lock = state.write().expect("lock poisoned");
    let me = lock.consensus.node_id;

    if !lock.consensus.relayed.contains(&payload.id) {
        lock.consensus.relayed.insert(payload.id.clone());

        let mut application = lock.applicaton.clone();
        lock.consensus.buffer.insert(payload.clone());
        lock.consensus.deliver_eligible_messages(&mut application);
        lock.applicaton = application.clone();
        drop(lock); // 🫡
        // -- end critical section

        info!("application state {:?}", application);
        perform_broadcast(&payload, me);
    }

    StatusCode::CREATED
}

pub fn perform_broadcast(broadcast_message: &BroadcastMessage, sender: usize) {
    for node_id in [sender, (sender + 1) % 5, (sender + 2) % 5] {
        let client = reqwest::Client::new();
        let broadcast_message = broadcast_message.clone();
        tokio::spawn(async move {
            if let Ok(response) = client
                .post(format!("http://0.0.0.0:{}/receive", 3000 + node_id))
                .json(&broadcast_message)
                .send()
                .await
            {
                if response.error_for_status().is_ok() {
                    tracing::info!("[broadcast] {} ---> {}", sender, node_id);
                }
            };
        });
    }
}
