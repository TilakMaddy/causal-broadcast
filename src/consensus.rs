use crate::{
    app::ApplicationState,
    serve::{BroadcastMessage, MessageIdentifier},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct ConsensusState {
    pub node_id: usize,
    pub send_seq: usize,
    pub delivered: [usize; 5],
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
    pub fn deliver_eligible_messages(&mut self, application: &mut ApplicationState) {
        loop {
            let mut try_again = false;
            let mut remove_indices = vec![];

            for (index, buffer) in self.buffer.clone().iter().enumerate() {
                // check if buffer is eligible for delivery
                let mut buffer_qualifies = (0..5)
                    .into_iter()
                    .all(|i| buffer.id.deps[i] <= self.delivered[i]);

                // if it is, deliver the message
                if buffer_qualifies {
                    application.deliver_message(buffer.message.clone());
                    self.delivered[buffer.id.sender] += 1;
                    remove_indices.push(index);
                    try_again = true;
                }
            }

            let mut new_buffer: BTreeSet<_> = Default::default();
            for (index, buffer) in self.buffer.clone().into_iter().enumerate() {
                if !remove_indices.iter().any(|&r| r == index) {
                    new_buffer.insert(buffer);
                }
            }

            std::mem::swap(&mut self.buffer, &mut new_buffer);

            if !try_again {
                break;
            }
        }
    }
}
