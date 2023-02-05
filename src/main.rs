use barakah::*;
use sailfish::TemplateOnce;
use std::{net::SocketAddr, sync::Arc};
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .with_state(Arc::new(Database::new().await?));
    
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
 
async fn index(db: Database) -> Result<Html<String>> {
    Ok(Html(template::Index.render_once()?)
}

