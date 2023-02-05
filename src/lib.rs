pub use sailfish::TemplateOnce;
           
pub mod template {
    use super::*;
    
    #[derive(TemplateOnce)]
    #[template(path = "index.html")]
    pub struct Index;
}
