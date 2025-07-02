use std::env::current_dir;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::system::FullSystemStateLocked;

// User -> Node (request to broadcast message)
#[derive(Debug, Deserialize)]
pub struct BroadcastRequestMessage {
    pub message: String,
}

// Node -> Node (broadcast)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastMessage {
    pub sender: usize,
    pub message: String,
    pub deps: [usize; 5],
}

#[tracing::instrument]
pub async fn broadcast_message(
    State(state): State<FullSystemStateLocked>,
    Json(payload): Json<BroadcastRequestMessage>,
) -> StatusCode {
    trace!("[broadcast request received]");

    let mut lock = state.write().expect("RW lock poisoned");
    let current_node_id = lock.consensus.node_id;
    let deps = {
        let mut received = lock.consensus.received;
        received[current_node_id] = lock.consensus.send_seq;
        received
    };
    lock.consensus.send_seq += 1;
    drop(lock); // 🫡

    let broadcast_message = BroadcastMessage {
        sender: current_node_id,
        message: payload.message,
        deps,
    };

    // Reliable broadcast
    for node_id in 0..5 {
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
                            tracing::trace!("[broadcast] {} ---> {}", current_node_id, node_id);
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

    StatusCode::CREATED
}

#[tracing::instrument]
pub async fn receive_message(
    State(state): State<FullSystemStateLocked>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    trace!("[broadcast message received]");
    let mut lock = state.write().expect("RW lock poisoned");

    StatusCode::CREATED
}
