use axum::{extract::FromRequest, response::Html, routing::get, Router};
use html_node::{html, Node};
pub mod index;
pub mod posts;

pub fn route() -> Router {
    let layout = Layout {
        parents: None,
        layout: layout,
    };
    let page = Page {
        tree: page(),
        parents: layout.clone()
    };
    Router::new()
        .route("/", get(page.render()))
        .nest("/posts", posts::router(layout)) // if the request doesnt match
                                               // it is routed to a subroute by
                                               // instantiating the router with
                                               // its parent tree
}

fn render(children: Node) -> Html<String> {
    Html(layout(children).to_string())
}

pub fn layout(children: Node) -> Node {
    html! {
        <div>
            {children}
        </div>
    }
}

pub fn page() -> Node {
    html! {
        <test>"this is not a test"</test>
        <p>"hello world"</p>
    }
}

#[derive(Clone)]
struct Layout {
    parents: Option<Box<Layout>>, // all the previous layouts
    layout: fn(&Node) -> Node,     // a function that takes a node and wraps it in a layout
}

impl Layout {
    fn render(&self, children: &Node) -> Html<String> {
        match &self.parents {
            Some(x) => x.render(&(self.layout)(children)),
            None => Html((self.layout)(children).to_string())
        }
    }
}

struct Page {
    tree: Node,
    parents: Layout
}

impl Page {
    /// takes the constructed render graph and outputs it to an html response
    fn render(&self) -> Html<String> {
        self.parents.render(&self.tree)
    }
}
