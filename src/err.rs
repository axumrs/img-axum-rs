use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::view;

#[derive(Debug)]
pub enum Kind {
    Config,
    Template,
    Image,
    S3,
    Hash,
    Database,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(kind: Kind, message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }

    pub fn with_cause(kind: Kind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }

    pub fn from_str(kind: Kind, msg: &str) -> Self {
        Self::new(kind, msg.to_string(), None)
    }
    pub fn from_string(kind: Kind, msg: String) -> Self {
        Self::new(kind, msg, None)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let tpl = view::ImageTemplate::error(self);
        let html = tpl.render().map_err(Self::from).unwrap();
        Html(html).into_response()
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::with_cause(Kind::Config, Box::new(e))
    }
}

impl From<askama::Error> for Error {
    fn from(e: askama::Error) -> Self {
        Self::with_cause(Kind::Template, Box::new(e))
    }
}

impl From<s3::error::S3Error> for Error {
    fn from(e: s3::error::S3Error) -> Self {
        Self::with_cause(Kind::S3, Box::new(e))
    }
}

impl From<base16ct::Error> for Error {
    fn from(e: base16ct::Error) -> Self {
        Self::from_string(Kind::Hash, e.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::with_cause(Kind::Database, Box::new(e))
    }
}
