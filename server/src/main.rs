use app::state::AppState;
use app::*;
use axum::{
    body::Body as AxumBody,
    extract::{Path, RawQuery, State},
    http::{header::HeaderMap, Request},
    response::IntoResponse,
};
use axum::{routing::post, Router};
use dotenv;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use std::{env, sync::Arc};

pub mod fileserv;

async fn handle_server_fns_with_state(
    State(state): State<AppState>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            provide_context(cx, state.clone());
        },
        request,
    )
    .await
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    dotenv::dotenv().ok();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // Load model
    let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");
    let model = llm::load::<llm::models::Llama>(
        std::path::Path::new(&model_path),
        llm::TokenizerSource::Embedded, // TODO: use HF tokenizers
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    let state = AppState {
        leptos_options,
        model: Arc::new(model),
        model_path,
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(handle_server_fns_with_state))
        .leptos_routes(&state, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
