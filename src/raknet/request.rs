use super::Message;

pub struct RakNetRequest {
    message: Message,
}

impl RakNetRequest {
    pub fn new(message: Message) -> Self {
        Self {
            message
        }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}