use crate::{serve::BroadcastMessage, *};

#[derive(Debug, Clone, Default)]
pub struct FullSystemState {
    pub consensus: ConsensusState,
    pub applicaton: ApplicationState,
}

pub type FullSystemStateLocked = Arc<RwLock<FullSystemState>>;

impl FullSystemState {
    pub fn new(node_id: usize) -> FullSystemState {
        Self {
            consensus: ConsensusState::new(node_id),
            ..Default::default()
        }
    }
    pub fn new_in_rwlock(node_id: usize) -> FullSystemStateLocked {
        let inner_state = FullSystemState::new(node_id);
        Arc::new(RwLock::new(inner_state.clone()))
    }
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
