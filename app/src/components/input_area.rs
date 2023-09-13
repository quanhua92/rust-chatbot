use crate::models::Conversation;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn InputArea(
    cx: Scope,
    submit: Action<String, Result<Conversation, ServerFnError>>,
) -> impl IntoView {
    let text_ref = create_node_ref::<Input>(cx);
    view! {
        cx,
        <form class="input-area" on:submit=move |ev| {
            ev.prevent_default();
            let input = text_ref.get().expect("input exists");
            let user_input = input.value();
            let user_input = user_input.trim();
            if !user_input.is_empty() {
                submit.dispatch(user_input.to_string());
                input.set_value("");
            }
        }>
            <input type="text" class="input-area-text" placeholder="Enter a prompt here" node_ref=text_ref/>
            <input type="submit" class="input-area-button" value="Send"/>
        </form>
    }
}
