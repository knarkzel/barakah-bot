use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Form, Router,
};
use barakah::{utils::*, *};
use rusqlite::params;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

async fn handle_error(_: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Static files
    let folder = concat!(env!("CARGO_MANIFEST_DIR"), "/static");
    let static_files = get_service(ServeDir::new(folder)).handle_error(handle_error);

    let app = Router::new()
        .route("/", get(index))
        .route("/comments", get(comments))
        .route("/channels", get(channels).post(insert_channel))
        .route("/channels/delete/:id", get(delete_channel))
        .nest_service("/static", static_files)
        .with_state(Arc::new(Database::new().await?));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn index() -> Result<Html<String>> {
    let template = template::Index;
    Ok(Html(template.render_once()?))
}

// Channels
async fn channels(State(db): State<Arc<Database>>) -> Result<impl IntoResponse> {
    // Fetch channels
    let channels = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, channel_id FROM channel")?;
            let channels = stmt
                .query_map([], |row| {
                    Ok(types::Channel {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        channel_id: row.get(2)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok::<_, rusqlite::Error>(channels)
        })
        .await?;

    let template = template::Channels { channels };
    Ok(Html(template.render_once()?))
}

async fn insert_channel(
    State(db): State<Arc<Database>>,
    Form(channel): Form<types::ChannelForm>,
) -> Result<impl IntoResponse> {
    // Insert into database
    db.call(move |conn| {
        conn.execute(
            "INSERT INTO channel (name, channel_id) VALUES (?1, ?2)",
            params![channel.name, channel.channel_id],
        )
    })
    .await?;

    // Redirect back to channels
    Ok(Redirect::to("/channels"))
}

async fn delete_channel(
    State(db): State<Arc<Database>>,
    Path(id): Path<usize>,
) -> Result<impl IntoResponse> {
    // Remove from database
    db.call(move |conn| conn.execute("DELETE FROM channel WHERE id = (?1)", params![id])).await?;

    // Redirect back to channels
    Ok(Redirect::to("/channels"))
}

// Comments
async fn comments(Query(video): Query<types::Video>) -> Result<impl IntoResponse> {
    let body = match video.video_id {
        Some(id) => fetch_comments(id).await?,
        _ => String::new(),
    };
    let template = template::Comments { body };
    Ok(Html(template.render_once()?))
}
