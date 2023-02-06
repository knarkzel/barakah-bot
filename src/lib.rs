// Imports
pub use axum_error::Result;

// Modules
pub mod utils;

// Types
pub mod types {
    use serde::Deserialize;

    pub struct Channel {
        pub id: usize,
        pub name: String,
        pub channel_id: String,
    }

    #[derive(Deserialize)]
    pub struct ChannelForm {
        pub name: String,
        pub channel_id: String,
    }
    
    #[derive(Deserialize)]
    pub struct Video {
        pub video_id: Option<String>,
    }
}

// Templates
pub use sailfish::TemplateOnce;

pub mod template {
    use super::*;

    #[derive(TemplateOnce)]
    #[template(path = "index.html")]
    pub struct Index;

    #[derive(TemplateOnce)]
    #[template(path = "comments.html")]
    pub struct Comments {
        pub body: String,
    }

    #[derive(TemplateOnce)]
    #[template(path = "channels.html")]
    pub struct Channels {
        pub channels: Vec<types::Channel>,
    }
}

// Database
use tokio_rusqlite::Connection;

pub struct Database(Connection);

impl Database {
    pub async fn new() -> Result<Self> {
        // Get connection
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/database.sqlite");
        let connection = Connection::open(database).await?;

        // Create tables
        let tables = concat!(env!("CARGO_MANIFEST_DIR"), "/tables");
        for file in std::fs::read_dir(tables)? {
            let body = std::fs::read_to_string(file?.path())?;
            connection.call(move |db| db.execute(&body, [])).await?;
        }

        Ok(Self(connection))
    }
}

impl std::ops::Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
