use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PostMetaData {
    pub title: String,
}

pub fn list_posts(posts_dir: &str) -> Vec<PostMetaData> {
    let mut metadata: Vec<PostMetaData> = Vec::new();
    match std::fs::read_dir(posts_dir) {
        Err(e) => {
            log::warn!("{:?}", e);
        }
        Ok(paths) => {
            log::trace!("{:?}", paths);
            for path in paths {
                let mut file = File::open(path.unwrap().path()).unwrap();
                log::trace!("{:?}", file);
                let mut cont = String::new();
                log::trace!("{:?}", cont);
                let n = file.read_to_string(&mut cont).unwrap();
                log::trace!("{:?}", n);
                let (front_matter, content) =
                    serde_frontmatter::deserialize::<PostMetaData>(&cont).unwrap();
                log::trace!("{:?}", front_matter);
                log::trace!("{:?}", content);
                metadata.push(front_matter);
            }
        }
    };
    metadata
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success() {
        let _ = pretty_env_logger::try_init();
        let metadata = list_posts("./example/posts");
        assert!(metadata[0].title.eq("sample 1"));
        assert!(metadata[1].title.eq("sample 2"));
    }
    #[test]
    fn not_exists() {
        let _ = pretty_env_logger::try_init();
        let metadata = list_posts("./this file does not exists");
        assert!(metadata.is_empty());
    }
}
