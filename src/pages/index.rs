use axum::response::{Html, IntoResponse};
use html_node::html;

pub async fn page() -> impl IntoResponse {
    Html(
        html! {
                <div>"this is a test"</div>
        }
        .to_string(),
    )
}
