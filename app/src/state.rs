use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use leptos::LeptosOptions;
use axum::extract::FromRef;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

    }
}
