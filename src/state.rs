use std::sync::Arc;

use crate::ImgConfig;

pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
    pub bucket: Arc<s3::Bucket>,
    pub img_cfg: Arc<ImgConfig>,
}
