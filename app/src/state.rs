use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use leptos::LeptosOptions;
use axum::extract::FromRef;
use llm::models::Llama;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub model: Arc<Llama>,
    pub model_path: String,
}

    }
}
