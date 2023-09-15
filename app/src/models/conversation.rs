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
        // let messages = vec![Message {
        //     text: "How may I help you?".to_string(),
        //     sender: "AI".to_string(),
        // }];
        Self {
            messages: Vec::new(),
        }
    }
}
