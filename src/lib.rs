<<<<<<< Updated upstream
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(pub T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

pub mod assets;
pub mod routes;
=======
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Router,
};
use html_node::{html, text};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod assets;
pub mod examples;
pub mod routes;

pub async fn serve(router: Router, port: i32) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

fn test_macro() -> String {
    let doc = html! {
        <div>
            <h1>"Hello World"</h1>
        </div>
    };
    doc.to_string()
}
>>>>>>> Stashed changes
