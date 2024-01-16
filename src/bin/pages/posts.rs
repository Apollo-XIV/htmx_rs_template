use axum::{response::IntoResponse, routing::get, Router};
use html_node::{html, Node};
use htmx_rs_template::PageRoute;

pub fn router(parent: PageRoute) -> Router {
    PageRoute {
        page,
        ..PageRoute::default(parent)
    }
    .route("/delete", get(delete))
    .build()
}

fn page() -> Node {
    html! {
        <h2>"testing again"</h2>
    }
}

async fn delete() -> impl IntoResponse {
    "Test response from the server"
}
