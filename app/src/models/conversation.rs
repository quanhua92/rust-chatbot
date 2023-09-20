use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub text: String,
    pub sender: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub id: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect(),
            messages: Vec::new(),
        }
    }
}
