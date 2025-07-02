use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

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
    tracing::info!("broadcasting message! {:?}", payload);

    let mut lock = state.write().expect("RW lock poisoned");

    let deps = {
        let mut received = lock.consensus.received;
        received[lock.consensus.node_id] = lock.consensus.send_seq;
        received
    };
    let broadcast_message = BroadcastMessage {
        sender: lock.consensus.node_id,
        message: payload.message,
        deps,
    };

    lock.consensus.send_seq += 1;

    drop(lock); // 🫡

    // Reliable broadcast
    for node_id in 0..5 {
        let client = reqwest::Client::new();
        let broadcast_message = broadcast_message.clone();
        tokio::spawn(async move {
            _ = client
                .post(format!("localhost:{}/receive", 3001 + node_id))
                .json(&broadcast_message)
                .send()
                .await;
        });
    }

    StatusCode::CREATED
}

#[tracing::instrument]
pub async fn receive_message(
    State(state): State<FullSystemStateLocked>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    tracing::info!("received message! {:?}", payload);

    let mut lock = state.write().expect("RW lock poisoned");

    StatusCode::CREATED
}
