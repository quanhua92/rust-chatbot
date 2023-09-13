use crate::models::Conversation;
use leptos::*;

#[component]
pub fn ConversationArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        cx,
        <div>
            { move || conversation.get().messages.iter().map(move |message| {
                view! { cx,
                    <div>
                        {message.sender.clone()}: {message.text.clone()}
                    </div>
                }
            })
            .collect::<Vec<_>>()
            }

        </div>
    }
}
