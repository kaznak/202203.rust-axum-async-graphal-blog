use crate::backend::post::{Backend, PostData};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
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
fn read_post_path(path: &Path) -> Result<PostData, ()> {
    let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
    let file_opt = File::open(path);
    if let Err(e) = file_opt {
        log::warn!("{:?}", e);
        return Err(());
    }

    let mut file = file_opt.unwrap();
    let mut cont = String::new();
    let _n = file.read_to_string(&mut cont).unwrap();
    let (front_matter, content) = serde_frontmatter::deserialize::<PostFrontMatter>(&cont).unwrap();
    let PostFrontMatter { title } = front_matter;
    let postdata = PostData {
        title,
        slug,
        content: content.trim().to_string(),
    };
    log::trace!("{:?}", postdata);
    Ok(postdata)
}

fn build_write_data(filebackend: &FileBackend, postdata: &PostData) -> (PathBuf, String) {
    // make data
    let PostData {
        title,
        slug,
        content,
    } = postdata;
    let front_matter = PostFrontMatter {
        title: title.clone(),
    };
    let markdown = serde_frontmatter::serialize(front_matter, content.trim()).unwrap();
    let path = filebackend.slug_to_path(slug);
    (path, markdown)
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
    fn create_post(&self, postdata: &PostData) -> Result<PostData, ()> {
        let (path, markdown) = build_write_data(self, postdata);
        // write
        match File::create(path) {
            Ok(mut file) => {
                let _n = file.write(markdown.as_bytes());
                let postdata = postdata.clone();
                log::trace!("{:?}", postdata);
                Ok(postdata)
            }
            Err(e) => {
                log::warn!("{:?}", e);
                Err(())
            }
        }
    }
    /// Read
    fn read_post(&self, slug: &str) -> Result<PostData, ()> {
        let path = self.slug_to_path(slug);
        read_post_path(&path)
    }
    /// List
    fn list_posts(&self) -> Result<Vec<PostData>, ()> {
        let mut post_vec: Vec<PostData> = Vec::new();
        let FileBackend { posts_dir_path } = self;
        match std::fs::read_dir(posts_dir_path) {
            Err(e) => {
                log::warn!("{:?}", e);
                return Err(());
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
        Ok(post_vec)
    }
    /// Update
    fn update_post(&self, postdata: &PostData) -> Result<PostData, ()> {
        let (path, markdown) = build_write_data(self, postdata);
        // write
        match OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(true)
            .open(path)
        {
            Ok(mut file) => {
                let _n = file.write(markdown.as_bytes());
                let postdata = postdata.clone();
                log::trace!("{:?}", postdata);
                Ok(postdata)
            }
            Err(e) => {
                log::warn!("{:?}", e);
                Err(())
            }
        }
    }
    /// Delete
    fn delete_post(&self, postdata: &PostData) -> Result<(), ()> {
        let slug = &postdata.slug;
        let path = self.slug_to_path(&slug);
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(e) => {
                log::warn!("{:?}", e);
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_post_success() {
        let _ = pretty_env_logger::try_init();
        let posts_dir = "./example/posts";
        let filebackend = FileBackend::new(posts_dir);
        let slug = "sample3";

        // prepare
        let path = filebackend.slug_to_path(slug);
        let _ = std::fs::remove_file(path);

        // check before create
        let readdata_before = filebackend.read_post(&slug);
        assert!(readdata_before.is_err());

        // create
        let createdata = PostData {
            title: String::from("Sample Post 3"),
            slug: String::from(slug),
            content: String::from("a test body"),
        };
        log::trace!("createdata: {:?}", createdata);
        let retdata = filebackend.create_post(&createdata).unwrap();
        log::trace!("retdata: {:?}", retdata);
        assert!(retdata.eq(&createdata));

        // check after create
        let readdata = filebackend.read_post(slug).unwrap();
        log::trace!("readdata: {:?}", readdata);
        assert!(readdata.eq(&createdata));

        // finalize
        let path = filebackend.slug_to_path(slug);
        let _ = std::fs::remove_file(path);
    }
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
        let post_vec = filebackend.list_posts().unwrap();
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
        assert!(metadata.is_err());
    }
}
