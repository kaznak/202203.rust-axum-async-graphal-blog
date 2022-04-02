/// Post のデータ
#[derive(Clone, PartialEq, Debug)]
pub struct PostData {
    pub slug: String,
    pub title: String,
    pub content: String,
}

/// Backend の　trait
pub trait Backend {
    /// Create
    fn create_post(&self, postdata: &PostData) -> Result<PostData, Box<dyn std::error::Error>>;
    /// Read
    fn read_post(&self, slug: &str) -> Result<PostData, Box<dyn std::error::Error>>;
    /// List
    fn list_posts(&self) -> Result<Vec<PostData>, Box<dyn std::error::Error>>;
    /// Update
    fn update_post(&self, postdata: &PostData) -> Result<PostData, Box<dyn std::error::Error>>;
    /// Delete
    fn delete_post(&self, slug: &str) -> Result<(), Box<dyn std::error::Error>>;
}
