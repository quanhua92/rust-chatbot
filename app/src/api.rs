use crate::models::Conversation;
use crate::models::Message;
use leptos::*;

#[server(ProcessConversation "/api")]
pub async fn process_conversation(
    cx: Scope,
    conversation: Conversation,
) -> Result<Conversation, ServerFnError> {
    use crate::snapshot::{read_session, write_session};
    use crate::state::AppState;
    use llm::Model;
    use std::path::Path;

    let session_path = format!("/tmp/chat_snapshot_{}", conversation.id);
    let session_path = Path::new(&session_path);

    let state: AppState = use_context::<AppState>(cx)
        .ok_or(ServerFnError::ServerError("No server state".to_string()))?;

    let model = state.model;
    let prelude = r#"A chat between a human ("User") and an AI assistant ("AI"). The AI assistant gives helpful, detailed, and polite answers to the human's questions."#;
    let stop_sequence = "User:";
    let maximum_token_count = 50;

    let mut output: String = String::new();
    let mut buffer: String = String::new();
    log!("Loading session from {session_path:?}...");
    let (mut session, prompt) = match read_session(&model, session_path) {
        Ok(s) => {
            println!("Loaded session from {session_path:?}");
            let mut prompt = String::new();
            if let Some(m) = conversation.messages.last() {
                prompt.push_str(format!("{}: {}\n", m.sender, m.text).as_str());
            }
            prompt.push_str(format!("AI:").as_str());
            (s, prompt)
        }
        Err(_) => {
            let mut prompt = format!("{prelude}\n").to_string();
            for m in conversation.messages.clone() {
                prompt.push_str(format!("{}: {}\n", m.sender, m.text).as_str());
            }
            prompt.push_str(format!("AI:").as_str());
            let session_config = llm::InferenceSessionConfig {
                n_threads: num_cpus::get_physical(),
                ..Default::default()
            };
            println!("Create new session with config {session_config:?}");
            (model.start_session(session_config), prompt)
        }
    };
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

    unsafe {
        write_session(session, session_path);
    }

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
