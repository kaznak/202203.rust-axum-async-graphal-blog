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
    fn create_post(&self, postdata: &PostData) -> Result<PostData, ()>;
    /// Read
    fn read_post(&self, slug: &str) -> Result<PostData, ()>;
    /// List
    fn list_posts(&self) -> Result<Vec<PostData>, ()>;
    /// Update
    fn update_post(&self, postdata: &PostData) -> Result<PostData, ()>;
    /// Delete
    fn delete_post(&self, slug: &str) -> Result<(), ()>;
}
