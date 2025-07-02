use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::serve::BroadcastMessage;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageIdentifier {
    pub sender: usize,
    pub deps: [usize; 5],
}

#[derive(Debug, Clone, Default)]
pub struct ConsensusState {
    pub node_id: usize,
    pub send_seq: usize,
    pub received: [usize; 5],
    pub buffer: BTreeSet<BroadcastMessage>,
    pub relayed: BTreeSet<MessageIdentifier>,
}

impl ConsensusState {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            ..Default::default()
        }
    }
}
