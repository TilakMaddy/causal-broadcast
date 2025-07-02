use crate::*;

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
