// Imports
pub use axum_error::Result;

// Templates
pub use sailfish::TemplateOnce;
           
pub mod template {
    use super::*;
    
    #[derive(TemplateOnce)]
    #[template(path = "index.html")]
    pub struct Index;
}

// Database
use tokio_rusqlite::Connection;

pub struct Database(Connection);

impl Database {
    pub async fn new() -> Result<Self> {
        let connection = Connection::open_in_memory().await?;
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
