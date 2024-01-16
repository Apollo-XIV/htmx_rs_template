use axum::routing::{get, MethodRouter};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Router,
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
    pub router: Option<Router>,
}

impl PageRoute {
    pub fn build(&self) -> Router {
        self.to_owned()
            .router
            .unwrap_or(Router::new().route("/", get("Hello world")))
    }

    pub fn route(&mut self, path: &str, handler: MethodRouter) -> PageRoute {
        PageRoute {
            router: Some(
                self.router
                    .clone()
                    .unwrap_or(Router::new().route("/", get(self.render())))
                    .route(path, handler),
            ),
            ..self.to_owned()
        }
    }

    pub fn nest(&mut self, path: &str, router: fn(PageRoute) -> Router) -> PageRoute {
        PageRoute {
            router: Some(
                self.router
                    .clone()
                    .unwrap_or(Router::new())
                    .nest(path, router(self.clone())),
            ),
            page: self.page,
            parents: self.parents.clone(),
            layout: self.layout,
        }
    }

    pub fn render(&self) -> Html<String> {
        match &self.parents {
            Some(x) => x.embed((self.layout)((self.page)())),
            None => Html((self.layout)((self.page)()).to_string()),
        }
    }

    fn embed(&self, children: Node) -> Html<String> {
        match &self.parents {
            Some(x) => x.embed((self.layout)(children)),
            None => Html((self.layout)(children).to_string()),
        }
    }

    pub fn root() -> PageRoute {
        PageRoute {
            page: || html!(<h1>"Error 404"</h1>),
            layout: |children: Node| html!(<>{children}</>),
            parents: None,
            router: None,
        }
    }

    pub fn default(parents: PageRoute) -> PageRoute {
        PageRoute {
            page: || html!(<h1>"Error 404"</h1>),
            layout: |children: Node| html!(<>{children}</>),
            parents: Some(Box::new(parents)),
            router: None,
        }
    }
}
