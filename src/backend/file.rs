use std::{fs::File, io::Read};

pub fn list_posts(posts_dir: &str) {
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
                let _n = file.read_to_string(&mut cont).unwrap();
                log::trace!("> {:?}", cont);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        pretty_env_logger::init();
        list_posts("./example/posts");
        list_posts("./this file does not exists");
    }
}
