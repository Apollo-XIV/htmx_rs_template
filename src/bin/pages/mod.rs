use axum::{response::Html, routing::get, Router};
use html_node::{html, Node};
pub mod index;
pub mod posts;
use htmx_rs_template::PageRoute;

pub fn route() -> Router {
    let root = PageRoute {
        page,
        layout,
        ..PageRoute::default()
    };
    Router::new()
        .route("/", get(root.render()))
        .nest("/posts", posts::router(root)) 
}

pub fn layout(children: Node) -> Node {
    html! {
        <nav>
            <h1>"My Epic Site Title"</h1>
        </nav>
        <div>
            {children}
        </div>
    }
}

pub fn page() -> Node {
    html! {
        <test>"Website Root"</test>
        <p>"hello world"</p>
    }
}

