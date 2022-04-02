pub type Slug<'a> = &'a str;

/// Post のデータ
#[derive(Clone, PartialEq, Debug)]
pub struct PostData<'a> {
    pub slug: Slug<'a>,
    pub title: &'a str,
    pub content: &'a str,
}

/// Backend の　trait
pub trait Backend {
    /// Create
    fn create_post<'a>(
        &self,
        postdata: &'a PostData,
    ) -> Result<PostData<'a>, Box<dyn std::error::Error>>;
    /// Read
    fn read_post<'a>(&self, slug: &'a str) -> Result<PostData<'a>, Box<dyn std::error::Error>>;
    /// List
    fn list_posts<'a>(&self) -> Result<Vec<Slug>, Box<dyn std::error::Error>>;
    /// Update
    fn update_post<'a>(
        &self,
        postdata: &'a PostData,
    ) -> Result<PostData<'a>, Box<dyn std::error::Error>>;
    /// Delete
    fn delete_post(&self, slug: &str) -> Result<(), Box<dyn std::error::Error>>;
}
