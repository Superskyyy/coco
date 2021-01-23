use std::path::{Path, PathBuf};

use url::Url;

use crate::settings::Settings;

pub fn from(text: &str) -> String {
    let uri_path = match Url::parse(text) {
        Ok(url) => url,
        Err(e) => panic!("failed to parsed: {}", e),
    };

    let paths = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    return format!("{}.{}", paths.last().unwrap(), "json");
}

pub fn uri_to_path(url: &str) -> PathBuf {
    let uri_path = match Url::parse(url) {
        Ok(url) => url,
        Err(_e) => {
            return PathBuf::from(url);
        }
    };

    let root = Path::new(Settings::root_dir());
    let mut buf = root.join(PathBuf::from(uri_path.host().unwrap().to_string()));

    let segments = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    for path in segments {
        buf = buf.join(PathBuf::from(path));
    }

    buf
}

#[cfg(test)]
mod test {
    use crate::infrastructure::url_format::{from, uri_to_path};

    #[test]
    fn format_github_with_url_http() {
        let string = from("http://github.com/coco-rs/coco.fixtures");
        assert_eq!("coco.fixtures.json", string);
    }

    #[test]
    fn url_to_path() {
        let string = uri_to_path("http://github.com/coco-rs/coco.fixtures");
        assert_eq!(
            ".coco/github.com/coco-rs/coco.fixtures",
            string.to_str().unwrap()
        );
    }

    #[test]
    fn should_return_origin_when_is_git() {
        let string = uri_to_path(".coco/framework");
        assert_eq!(".coco/framework", string.to_str().unwrap());
    }
}
