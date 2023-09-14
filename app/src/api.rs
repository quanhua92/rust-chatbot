use crate::models::Conversation;
use crate::models::Message;
use leptos::*;

#[server(ProcessConversation "/api")]
pub async fn process_conversation(
    cx: Scope,
    conversation: Conversation,
) -> Result<Conversation, ServerFnError> {
    use crate::state::AppState;
    use llm::Model;

    let state: AppState = use_context::<AppState>(cx)
        .ok_or(ServerFnError::ServerError("No server state".to_string()))?;

    let model = state.model;

    let mut output: String = String::new();
    let mut session = model.start_session(Default::default());
    log!("Generating response...");
    let res = session.infer::<std::convert::Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: "The best method to make money with coding is".into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(50),
        },
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                output += &t;
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    println!("Output: \n\n{output}");

    match res {
        Ok(result) => println!("\n\nInference stats: \n {result}"),
        Err(err) => println!("\n{err}"),
    }

    log!("process_conversation {:?}", conversation);
    let mut conversation = conversation;

    conversation.messages.push(Message {
        text: output,
        sender: "AI".to_string(),
    });
    Ok(conversation)
}
