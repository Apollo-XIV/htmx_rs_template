use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use html_node::{html, Node};

pub mod assets;
pub mod components;
pub mod routes;

#[derive(Clone)]
pub struct PageRoute {
    pub page: fn() -> Node,
    pub parents: Option<Box<PageRoute>>, // all the previous layouts
    pub layout: fn(Node) -> Node,        // a function that takes a node and wraps it in a layout
}

impl PageRoute {
    pub fn render(&self) -> Html<String> {
        match &self.parents {
            Some(x) => x.nest((self.layout)((self.page)())),
            None => Html((self.layout)((self.page)()).to_string()),
        }
    }

    fn nest(&self, children: Node) -> Html<String> {
        match &self.parents {
            Some(x) => x.nest((self.layout)(children)),
            None => Html((self.layout)(children).to_string()),
        }
    }

    pub fn default() -> PageRoute {
        PageRoute {
            page: || html!(<h1>"Error 404"</h1>),
            layout: |children: Node| html!(<>{children}</>),
            parents: None,
        }
    }
}
