#[derive(Debug, Clone, Default)]
pub struct ApplicationState {
    pub messages: Vec<String>,
}

impl ApplicationState {
    pub fn deliver_message(&mut self, message: String) {
        self.messages.push(message);
    }
}
