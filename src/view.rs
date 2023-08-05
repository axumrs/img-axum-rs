use askama::Template;

use crate::{Error, ImgConfig};

#[derive(Template)]
#[template(path = "image.html")]
pub struct ImageTemplate<'a> {
    pub url: Option<String>,
    pub err: Option<Error>,
    pub cfg: Option<&'a ImgConfig>,
}

impl<'a> ImageTemplate<'a> {
    pub fn upload(cfg: &'a ImgConfig) -> Self {
        Self {
            url: None,
            err: None,
            cfg: Some(cfg),
        }
    }
    pub fn error(err: Error) -> Self {
        Self {
            url: None,
            err: Some(err),
            cfg: None,
        }
    }
    pub fn result(url: String) -> Self {
        Self {
            url: Some(url),
            err: None,
            cfg: None,
        }
    }
}
