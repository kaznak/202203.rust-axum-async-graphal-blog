
/// Post のデータ
#[derive(PartialEq, Debug)]
pub struct PostData {
    pub title: String,
    pub slug: String,
    pub content: String,
}

/// Backend の　trait
trait Backend {
    /// Create
    fn creates_post(&self, postdata: &PostData) -> Option<PostData>;
    /// Read
    fn read_post(&self, slug: &str) -> Option<PostData>;
    /// List
    fn list_posts(&self) -> Vec<PostData>;
    /// Update
    fn update_post(&self, postdata: &PostData) -> Option<PostData>;
    /// Delete
    fn delete_post(&self, postdata: &PostData) -> Option<PostData>;
}
