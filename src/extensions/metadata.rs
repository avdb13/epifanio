pub enum Message {
    Request,
    Data(Vec<u8>),
    Reject,
}

impl From<&Message> for u8 {
    fn from(parts: &Message) -> Self {
        match parts {
            Message::Request { .. } => 0,
            Message::Data { .. } => 1,
            Message::Reject { .. } => 2,
        }
    }
}
