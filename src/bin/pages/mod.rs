use axum::{response::Html, routing::get, Router};
use html_node::{html, Node};
pub mod index;
pub mod posts;
use htmx_rs_template::{components::navbar::navbar, PageRoute};

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
        <!doctype html>
        <html lang="en">
            <head>
                <link href="/assets/main.css" rel="stylesheet" />
                <link href="https://rsms.me/inter/inter.css" rel="stylesheet" />
                <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=IBM+Plex+Mono"/>
                <!-- "htmx from the unpkg CDN - your mileage may vary" -->
                <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            </head>
            <body class="bg-tasman-600">
                {navbar()}
                <div id="content">
                    <!-- "Inheriting pages will have their content rendered here, similar to app root in React, Angular, etc." -->
                    {children}
                </div>
            </body>
        </html>
    }
}

pub fn page() -> Node {
    html! {
        <test>"Website Root"</test>
        <p>"hello world"</p>
    }
}
