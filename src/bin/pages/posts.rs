use htmx_rs_template::PageRoute;
use axum::{response::IntoResponse, routing::get, Router};
use html_node::{html, Node};

pub fn router(parent: PageRoute) -> Router {
    let page = PageRoute {
        page,
        parents: Some(Box::new(parent)),
        ..PageRoute::default()
    };
    Router::new().route("/", get(page.render()))
}

fn page() -> Node {
    html! {
        <h2>"testing again"</h2>
    }
}
