use leptos::*;

use crate::models::Conversation;
use crate::models::Message;

#[server(ProcessConversation "/api")]
pub async fn process_conversation(
    conversation: Conversation,
) -> Result<Conversation, ServerFnError> {
    log!("process_conversation {:?}", conversation);
    let mut conversation = conversation;

    conversation.messages.push(Message {
        text: "Response from AI".to_string(),
        sender: "AI".to_string(),
    });
    Ok(conversation)
}
