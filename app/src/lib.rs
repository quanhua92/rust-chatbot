use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod error_template;
pub mod models;

use crate::components::{ConversationArea, InputArea};
use crate::models::Conversation;

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

    view! { cx,
        <div class="chat-area">
            <ConversationArea conversation />
            <InputArea />
        </div>
    }
}
