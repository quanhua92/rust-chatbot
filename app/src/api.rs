use crate::models::Conversation;
use crate::models::Message;
use leptos::*;

#[server(ProcessConversation "/api")]
pub async fn process_conversation(
    cx: Scope,
    conversation: Conversation,
) -> Result<Conversation, ServerFnError> {
    use crate::state::AppState;
    let state: AppState = use_context::<AppState>(cx)
        .ok_or(ServerFnError::ServerError("No server state".to_string()))?;

    log!("state: {state:?}");
    log!("process_conversation {:?}", conversation);
    let mut conversation = conversation;

    conversation.messages.push(Message {
        text: "Response from AI".to_string(),
        sender: "AI".to_string(),
    });
    Ok(conversation)
}
