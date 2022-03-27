use crate::backend::post::{Backend, PostData};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// Post の front matter のデータ
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PostFrontMatter {
    pub title: String,
}

/// Backend on file system
pub struct FileBackend {
    pub posts_dir_path: PathBuf,
}

/// Post を path で指定して読み出す。
fn read_post_path(path: &Path) -> Option<PostData> {
    let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
    let mut file = File::open(path).unwrap();
    let mut cont = String::new();
    let _n = file.read_to_string(&mut cont).unwrap();
    let (front_matter, content) = serde_frontmatter::deserialize::<PostFrontMatter>(&cont).unwrap();
    let PostFrontMatter { title } = front_matter;
    let postdata = PostData {
        title,
        slug,
        content,
    };
    Some(postdata)
}

impl FileBackend {
    /// Constructor
    pub fn new(posts_dir: &str) -> FileBackend {
        let posts_dir_path = Path::new(posts_dir).to_path_buf();
        FileBackend { posts_dir_path }
    }
    /// slug から path を作成する。
    fn slug_to_path(&self, slug: &str) -> PathBuf {
        let FileBackend { posts_dir_path } = self;
        let path = posts_dir_path.join(slug).with_extension("md");
        log::trace!("{:?}", path);
        path
    }
}

impl Backend for FileBackend {
    /// Create
    fn creates_post(&self, postdata: &PostData) -> Option<PostData> {
        // make data
        let PostData {
            title,
            slug,
            content,
        } = postdata;
        let front_matter = PostFrontMatter {
            title: title.clone(),
        };
        let markdown = serde_frontmatter::serialize(front_matter, content).unwrap();
        let path = self.slug_to_path(slug);

        // write
        todo!()
    }
    /// Read
    fn read_post(&self, slug: &str) -> Option<PostData> {
        let path = self.slug_to_path(slug);
        read_post_path(&path)
    }
    /// List
    fn list_posts(&self) -> Vec<PostData> {
        let mut post_vec: Vec<PostData> = Vec::new();
        let FileBackend { posts_dir_path } = self;
        match std::fs::read_dir(posts_dir_path) {
            Err(e) => {
                log::warn!("{:?}", e);
            }
            Ok(paths) => {
                // log::trace!("{:?}", paths);
                for direntry_result in paths {
                    let path = direntry_result.unwrap().path();
                    let postdata = read_post_path(&path).unwrap();
                    post_vec.push(postdata);
                }
            }
        };
        log::trace!("{:?}", post_vec);
        post_vec
    }
    /// Update
    fn update_post(&self, postdata: &PostData) -> Option<PostData> {
        todo!()
    }
    /// Delete
    fn delete_post(&self, postdata: &PostData) -> Option<PostData> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_post_success() {
        let _ = pretty_env_logger::try_init();
        let posts_dir = "./example/posts";
        let slug = "sample1";
        let filebackend = FileBackend::new(posts_dir);
        let post = filebackend.read_post(slug).unwrap();
        assert!(post.slug.eq("sample1"));
        assert!(post.title.eq("sample 1"));
    }
    #[test]
    fn list_posts_success() {
        let _ = pretty_env_logger::try_init();
        let posts_dir = "./example/posts";
        let filebackend = FileBackend::new(posts_dir);
        let post_vec = filebackend.list_posts();
        assert!(post_vec[0].slug.eq("sample1"));
        assert!(post_vec[0].title.eq("sample 1"));
        assert!(post_vec[1].slug.eq("sample2"));
        assert!(post_vec[1].title.eq("sample 2"));
    }
    #[test]
    fn list_posts_not_exists() {
        let _ = pretty_env_logger::try_init();
        let posts_dir = "./this file does not exists";
        let filebackend = FileBackend::new(posts_dir);
        let metadata = filebackend.list_posts();
        assert!(metadata.is_empty());
    }
}
