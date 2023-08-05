use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, routing::get, Router};
use img_axum_rs::{handler, AppState, Config, ImgConfig};
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env().unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.pg.max_connections)
        .connect(&cfg.pg.dsn)
        .await
        .unwrap();
    let pool = Arc::new(pool);

    let bucket = s3::Bucket::new(
        &cfg.s3.bucket,
        cfg.s3.region.parse().unwrap(),
        s3::creds::Credentials {
            access_key: Some(cfg.s3.access_key.clone()),
            secret_key: Some(cfg.s3.secret_key.clone()),
            session_token: None,
            security_token: None,
            expiration: None,
        },
    )
    .unwrap();
    let bucket = Arc::new(bucket);

    let img_cfg = Arc::new(cfg.img);

    tokio::spawn(remove_expired_objects(
        pool.clone(),
        bucket.clone(),
        img_cfg.clone(),
    ));

    let app = Router::new()
        .route("/", get(handler::upload_ui).post(handler::upload))
        .route("/:url", get(handler::result_ui))
        .with_state(Arc::new(AppState {
            pool,
            bucket,
            img_cfg: img_cfg.clone(),
        }))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new((&*img_cfg).max_size * 2));

    tracing::info!("Web服务监听于：{}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn remove_expired_objects(
    pool: Arc<sqlx::PgPool>,
    bucket: Arc<s3::Bucket>,
    img_cfg: Arc<ImgConfig>,
) {
    tracing::info!("开始删除过期对象");
}
