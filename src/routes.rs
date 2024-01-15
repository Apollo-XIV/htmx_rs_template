use axum::{response::IntoResponse, routing::get, Router};
use html_node::html;

// Top level subpages

pub fn router() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> impl IntoResponse {
    html! {
        <h1>"Hello World"</h1>
    }
    .to_string()
}
