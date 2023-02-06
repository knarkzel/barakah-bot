use barakah::*;
use rusqlite::params;
use std::{net::SocketAddr, sync::Arc};
use axum::{response::Html, routing::get, Router, extract::State};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let app = Router::new()
        .route("/", get(index))
        .with_state(Arc::new(Database::new().await?));
    
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
 
async fn index(State(db): State<Arc<Database>>) -> Result<Html<String>> {
    db.call(|conn| {
        conn.execute(
            "INSERT INTO person (name) VALUES (?1)",
            params!["James".to_owned()],
        )
    }).await?;
    
    Ok(Html(template::Index.render_once()?))
}
