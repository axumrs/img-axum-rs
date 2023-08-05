use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, routing::get, Router};
use img_axum_rs::{handler, AppState, Config, Error};
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir};

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

    let app_state = Arc::new(AppState {
        pool,
        bucket,
        img_cfg: img_cfg.clone(),
    });

    tokio::spawn(remove_expired_objects(app_state.clone()));

    let app = Router::new()
        .route("/", get(handler::upload_ui).post(handler::upload))
        .route("/:url", get(handler::result_ui))
        .with_state(app_state)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new((&*img_cfg).max_size * 2))
        .nest_service("/static", ServeDir::new("static"));

    tracing::info!("Web服务监听于：{}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn remove_expired_objects(app_state: Arc<AppState>) {
    loop {
        tracing::info!("开始删除过期对象");
        let expired_dateline =
            chrono::Local::now() - chrono::Duration::days((&app_state.img_cfg).expires_days as i64);
        // let sql = "UPDATE images SET is_deleted=true WHERE dateline <= $1 AND is_deleted=false RETURNING path";
        let sql = "DELETE FROM images WHERE dateline <= $1 RETURNING path";
        let paths: Vec<(String,)> = match sqlx::query_as(sql)
            .bind(&expired_dateline)
            .fetch_all(&*app_state.pool)
            .await
        {
            Ok(paths) => paths,
            Err(e) => {
                let e = Error::from(e);
                tracing::error!("查询过期对象失败：{}", e.message);
                continue;
            }
        };
        tracing::info!("共有{}个过期对象", paths.len());
        for path in paths {
            match (&*app_state.bucket).delete_object(&path.0).await {
                Ok(resp) => {
                    tracing::info!("删除过期对象成功：{}, {:?}", &path.0, resp);
                }
                Err(e) => {
                    let e = Error::from(e);
                    tracing::error!("删除过期对象失败：{}", e.message);
                }
            };
        }
        tracing::info!(
            "休息{}秒钟，马上回来",
            (&*app_state.img_cfg).remove_duration_secs
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(
            (&*app_state.img_cfg).remove_duration_secs as u64,
        ))
        .await;
    }
}
