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

/// Post のデータ
#[derive(PartialEq, Debug)]
pub struct PostData {
    pub title: String,
    pub slug: String,
    pub content: String,
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

/// slug から path を作成する。
fn slug_to_path(posts_dir: &str, slug: &str) -> PathBuf {
    let path = Path::new(posts_dir).join(slug).with_extension("md");
    log::trace!("{:?}", path);
    path
}

/// Post を slug で指定して読み出す。
pub fn read_post_slug(posts_dir: &str, slug: &str) -> Option<PostData> {
    let path = slug_to_path(posts_dir, slug);
    read_post_path(&path)
}

/// すべての Post を読み出し vector で返す。
pub fn list_posts(posts_dir: &str) -> Vec<PostData> {
    let mut post_vec: Vec<PostData> = Vec::new();
    match std::fs::read_dir(posts_dir) {
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

/// あたらしい Post を書き込む。
pub fn creates_post(posts_dir: &str, postdata: &PostData) -> bool {
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
    let path = slug_to_path(posts_dir, slug);

    // write
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_post_slug_success() {
        let _ = pretty_env_logger::try_init();
        let post = read_post_slug("./example/posts", "sample1").unwrap();
        assert!(post.slug.eq("sample1"));
        assert!(post.title.eq("sample 1"));
    }
    #[test]
    fn list_posts_success() {
        let _ = pretty_env_logger::try_init();
        let post_vec = list_posts("./example/posts");
        assert!(post_vec[0].slug.eq("sample1"));
        assert!(post_vec[0].title.eq("sample 1"));
        assert!(post_vec[1].slug.eq("sample2"));
        assert!(post_vec[1].title.eq("sample 2"));
    }
    #[test]
    fn list_posts_not_exists() {
        let _ = pretty_env_logger::try_init();
        let metadata = list_posts("./this file does not exists");
        assert!(metadata.is_empty());
    }
}
