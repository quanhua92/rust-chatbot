use crate::models::Conversation;
use leptos::html::Div;
use leptos::*;

#[component]
pub fn ConversationArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        let c = conversation.get();
        log!("ConversationArea: {:?}", c);
        if let Some(div) = div_ref.get() {
            request_animation_frame(move || {
                div.set_scroll_top(div.scroll_height());
            });
        }
    });

    view! {
        cx,
        <div class="conversation-area" node_ref=div_ref>
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
