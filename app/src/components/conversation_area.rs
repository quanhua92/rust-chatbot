use crate::models::Conversation;
use leptos::*;

#[component]
pub fn ConversationArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        cx,
        <div class="conversation-area">
            { move || conversation.get().messages.iter().map(move |message| {
                view! { cx,
                    <div class="message">
                        <span class="message-sender">{message.sender.clone()}</span>
                        <p class="message-text">{message.text.clone()}</p>
                    </div>
                }
            })
            .collect::<Vec<_>>()
            }

        </div>
    }
}
