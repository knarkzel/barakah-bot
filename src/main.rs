use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use barakah::*;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

async fn handle_error(_: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Static files
    let static_files = get_service(ServeDir::new("static")).handle_error(handle_error);

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", static_files)
        .with_state(Arc::new(Database::new().await?));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn index(State(db): State<Arc<Database>>) -> Result<Html<String>> {
    Ok(Html(template::Index.render_once()?))
}
