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
    let prelude = r#"A chat between a human ("User") and an AI assistant ("AI"). The AI assistant gives helpful, detailed, and polite answers to the human's questions."#;
    let mut prompt = format!("{prelude}\n").to_string();
    for message in conversation.messages.clone() {
        let sender = message.sender;
        let text = message.text;
        prompt.push_str(format!("{sender}: {text}\n").as_str());
    }
    prompt.push_str(format!("AI:").as_str());
    let stop_sequence = "User:";
    let maximum_token_count = 100;

    let mut output: String = String::new();
    let mut buffer: String = String::new();
    let mut session = model.start_session(llm::InferenceSessionConfig {
        n_threads: num_cpus::get_physical(),
        ..Default::default()
    });
    log!("Generating response...");
    log!("Prompt: {}", prompt);
    let res = session.infer::<std::convert::Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: prompt.as_str().into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(maximum_token_count),
        },
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::InferredToken(token) => {
                let mut buf = buffer.clone();
                buf.push_str(&token);

                if buf.starts_with(stop_sequence) {
                    buffer.clear();
                    return Ok(llm::InferenceFeedback::Halt);
                } else if stop_sequence.starts_with(&buf) {
                    buffer = buf;
                    return Ok(llm::InferenceFeedback::Continue);
                }
                buffer.clear();
                output.push_str(&buf);
                Ok(llm::InferenceFeedback::Continue)
            }
            llm::InferenceResponse::EotToken => Ok(llm::InferenceFeedback::Halt),
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    println!("Output: {output}");

    match res {
        Ok(result) => println!("\n\nInference stats: \n {result}"),
        Err(err) => println!("\n{err}"),
    }

    let mut conversation = conversation;
    conversation.messages.push(Message {
        text: output,
        sender: "AI".to_string(),
    });
    Ok(conversation)
}
