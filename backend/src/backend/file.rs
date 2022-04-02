use crate::backend::post::{Backend, PostData, Slug};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Post の front matter のデータ
#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct PostFrontMatter {
    pub title: String,
}

/// Backend on file system
pub struct FileBackend {
    pub posts_dir_path: PathBuf,
}

#[derive(PartialEq, Debug)]
enum FileBackendSpecificErrors {
    MissingFrontMatter,
}

impl std::fmt::Display for FileBackendSpecificErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileBackendSpecificErrors::MissingFrontMatter => write!(f, "{}", self),
        }
    }
}

impl std::error::Error for FileBackendSpecificErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// Post を path で指定して読み出す。
fn read_post_path(path: &Path) -> Result<PostData, Box<dyn std::error::Error>> {
    let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
    let mut file = File::open(path)?;

    let mut cont = String::new();
    let _n = file.read_to_string(&mut cont)?;
    let (front_matter, content) = match serde_frontmatter::deserialize::<PostFrontMatter>(&cont) {
        Ok(v) => v,
        Err(_) => return Err(Box::new(FileBackendSpecificErrors::MissingFrontMatter)),
    };
    let PostFrontMatter { title } = front_matter;
    let postdata = PostData {
        title,
        slug,
        content: content.trim().to_string(),
    };
    log::trace!("{:?}", postdata);
    Ok(postdata)
}

/// PostData からファイルシステム操作のためのデータを構築する
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
    fn create_post(&self, postdata: &PostData) -> Result<PostData, Box<dyn std::error::Error>> {
        let (path, markdown) = build_write_data(self, postdata);
        // write
        let mut file = File::create(path)?;
        let _n = file.write(markdown.as_bytes());
        let postdata = postdata.clone();
        log::trace!("{:?}", postdata);
        Ok(postdata)
    }
    /// Read
    fn read_post(&self, slug: &str) -> Result<PostData, Box<dyn std::error::Error>> {
        let path = self.slug_to_path(slug);
        read_post_path(&path)
    }
    /// List
    fn list_posts(&self) -> Result<Vec<Slug>, Box<dyn std::error::Error>> {
        let mut slug_vec: Vec<Slug> = Vec::new();
        let FileBackend { posts_dir_path } = self;
        let paths = std::fs::read_dir(posts_dir_path)?;
        for direntry_result in paths {
            let path = direntry_result?.path();
            slug_vec.push(path.file_stem().unwrap().to_str().unwrap().to_string());
        }
        Ok(slug_vec)
    }
    /// Update
    fn update_post(&self, postdata: &PostData) -> Result<PostData, Box<dyn std::error::Error>> {
        let (path, markdown) = build_write_data(self, postdata);
        // write
        let mut file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(true)
            .open(path)?;
        let _n = file.write(markdown.as_bytes());
        let postdata = postdata.clone();
        log::trace!("{:?}", postdata);
        Ok(postdata)
    }
    /// Delete
    fn delete_post(&self, slug: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.slug_to_path(&slug);
        let ret = std::fs::remove_file(path)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_post_delete_post_success() {
        let _ = pretty_env_logger::try_init();
        let filebackend = FileBackend::new("./example/posts");
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

        // delete
        let delresult = filebackend.delete_post(&createdata.slug);
        assert!(delresult.is_ok());
    }
    #[test]
    fn read_post_success() {
        let _ = pretty_env_logger::try_init();
        let filebackend = FileBackend::new("./example/posts");
        let slug = "sample1";
        let post = filebackend.read_post(slug).unwrap();
        assert!(post.slug.eq("sample1"));
        assert!(post.title.eq("sample 1"));
    }
    #[test]
    fn list_posts_success() {
        let _ = pretty_env_logger::try_init();
        let filebackend = FileBackend::new("./example/posts");
        let slug_vec = filebackend.list_posts().unwrap();
        eprintln!("{:?}", slug_vec);
        assert!(slug_vec[0].eq("sample1"));
        assert!(slug_vec[1].eq("sample2"));
    }
    #[test]
    fn list_posts_not_exists() {
        let _ = pretty_env_logger::try_init();
        let posts_dir = "./this file does not exists";
        let filebackend = FileBackend::new(posts_dir);
        let metadata = filebackend.list_posts();
        assert!(metadata.is_err());
    }
    #[test]
    fn update_post_success() {
        let _ = pretty_env_logger::try_init();
        let filebackend = FileBackend::new("./example/posts");
        let slug = "sample2";

        // check before update
        let readdata_before = filebackend.read_post(&slug);
        assert!(readdata_before.is_ok());
        let original_postdata = readdata_before.unwrap();
        let PostData {
            title,
            slug,
            content,
        } = original_postdata.clone();
        assert!(!content.eq("hoge"));

        // update
        let updatedata = PostData {
            title: title.clone(),
            slug: slug.clone(),
            content: String::from("hoge"),
        };
        log::trace!("createdata: {:?}", updatedata);
        let retdata = filebackend.update_post(&updatedata).unwrap();
        log::trace!("retdata: {:?}", retdata);
        assert!(retdata.eq(&updatedata));

        // check after create
        let readdata = filebackend.read_post(&slug).unwrap();
        log::trace!("readdata: {:?}", readdata);
        assert!(readdata.eq(&updatedata));

        // finalize
        let finiret = filebackend.update_post(&original_postdata).unwrap();
        assert!(finiret.eq(&original_postdata));
    }
}
