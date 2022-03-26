use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PostFrontMatter {
    pub title: String,
}

#[derive(PartialEq, Debug)]
pub struct PostMetaData {
    pub front_matter: PostFrontMatter,
    pub slug: String,
}

pub fn list_posts(posts_dir: &str) -> Vec<PostMetaData> {
    let mut postdata: Vec<PostMetaData> = Vec::new();
    match std::fs::read_dir(posts_dir) {
        Err(e) => {
            log::warn!("{:?}", e);
        }
        Ok(paths) => {
            // log::trace!("{:?}", paths);
            for direntry_result in paths {
                let path = direntry_result.unwrap().path();
                let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
                let mut file = File::open(path).unwrap();
                // log::trace!("{:?}", file);
                let mut cont = String::new();
                // log::trace!("{:?}", cont);
                let _n = file.read_to_string(&mut cont).unwrap();
                // log::trace!("{:?}", _n);
                let (front_matter, _content) =
                    serde_frontmatter::deserialize::<PostFrontMatter>(&cont).unwrap();
                // log::trace!("{:?}", front_matter);
                // log::trace!("{:?}", _content);
                let metadata = PostMetaData { front_matter, slug };
                postdata.push(metadata);
            }
        }
    };
    log::trace!("{:?}", postdata);
    postdata
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success() {
        let _ = pretty_env_logger::try_init();
        let metadata = list_posts("./example/posts");
        assert!(metadata[0].slug.eq("sample1"));
        assert!(metadata[0].front_matter.title.eq("sample 1"));
        assert!(metadata[1].slug.eq("sample2"));
        assert!(metadata[1].front_matter.title.eq("sample 2"));
    }
    #[test]
    fn not_exists() {
        let _ = pretty_env_logger::try_init();
        let metadata = list_posts("./this file does not exists");
        assert!(metadata.is_empty());
    }
}
