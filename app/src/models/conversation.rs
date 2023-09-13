use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub text: String,
    pub sender: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        let messages = vec![
            Message {
                text: "Hello from bot".to_string(),
                sender: "bot".to_string(),
            },
            Message {
                text: "Hello from user".to_string(),
                sender: "user".to_string(),
            },
        ];
        Self {
            messages, // messages: Vec::new(),
        }
    }
}
