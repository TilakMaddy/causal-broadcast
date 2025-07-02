#[derive(Clone, Debug, Default)]
pub struct BufferMessage {
    pub node_id: usize,
    pub dependencies: [usize; 5],
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConsensusState {
    pub node_id: usize,
    pub send_seq: usize,
    pub received: [usize; 5],
    pub buffer: Vec<BufferMessage>,
}

impl ConsensusState {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            ..Default::default()
        }
    }
}
