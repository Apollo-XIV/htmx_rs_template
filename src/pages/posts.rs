use super::Layout;
use super::Page;
use axum::{response::IntoResponse, routing::get, Router};
use html_node::{html, Node};

pub fn router(parent: Layout) -> Router {
    let page = Page {
        tree: page(),
        parents: parent,
    };
    Router::new().route("/", get(page.render()))
}

fn page() -> Node {
    html! {
        <h2>"testing again"</h2>
    }
}
