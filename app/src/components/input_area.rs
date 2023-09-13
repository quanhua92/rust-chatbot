use leptos::*;

#[component]
pub fn InputArea(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div style="display: flex">
            <textarea placeholder="Enter a prompt here"/>
            <button>Send</button>
        </div>
    }
}
