use barakah::*;
use axum_error::Result;
use std::net::SocketAddr;
use sailfish::TemplateOnce;
use axum::{response::Html, routing::get, Router};
 
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await
        .unwrap()
}
 
async fn index() -> Result<Html<String>> {
    Ok(Html(template::Index.render_once()?))
}
