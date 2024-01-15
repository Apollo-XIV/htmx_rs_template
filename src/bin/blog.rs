use axum::routing::get;
use axum::Router;
use htmx_rs_template::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/", get("Hello World"));
    serve(router, 3000).await.unwrap();
    Ok(())
}
