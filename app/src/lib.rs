use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod api;
pub mod components;
pub mod error_template;
pub mod models;
pub mod state;

use crate::api::process_conversation;
use crate::components::{ConversationArea, InputArea};
use crate::models::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Rust Chatbot"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());
    let send_message = create_action(cx, move |input: &String| {
        let message = Message {
            text: input.clone(),
            sender: "User".to_string(),
        };
        set_conversation.update(move |c| {
            c.messages.push(message);
        });

        process_conversation(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send_message.input().get() {
            set_conversation.update(move |c| {
                c.messages.push(Message {
                    text: "...".to_string(),
                    sender: "AI".to_string(),
                });
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send_message.value().get() {
            set_conversation.set(response);
        }
    });

    view! { cx,
        <div class="chat-area">
            <ConversationArea conversation />
            <InputArea submit=send_message />
        </div>
    }
}
