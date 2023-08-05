use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Html,
};

use crate::{meta, view, AppState, Error, Result};

type RedirectResponse = (StatusCode, HeaderMap, ());

pub async fn upload_ui(State(state): State<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "upload_ui";

    let tpl = view::ImageTemplate::upload(&state.img_cfg);
    let html = tpl
        .render()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Html(html))
}

pub async fn result_ui(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Result<Html<String>> {
    let handler_name = "result_ui";

    let url = format!("{}{}", &state.img_cfg.domain, url);

    let tpl = view::ImageTemplate::result(url);
    let html = tpl
        .render()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Html(html))
}

pub async fn upload(
    State(state): State<Arc<AppState>>,
    mut multipar: Multipart,
) -> Result<RedirectResponse> {
    let handler_name = "upload";
    if let Some(field) = multipar.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let data_len = data.len();

        if !(&content_type == "image/png"
            || &content_type == "image/jpeg"
            || &content_type == "image/gif")
        {
            let err = Err(Error::from_str(
                crate::ErrorKind::Image,
                "你只能上传PNG/JPG/GIF图片",
            ))
            .map_err(log_error(handler_name));
            return err;
        }

        if data_len > (&state.img_cfg).max_size {
            let err = Err(Error::from_str(
                crate::ErrorKind::Image,
                "文件大小超过允许的最大值",
            ))
            .map_err(log_error(handler_name));
            return err;
        }

        // 计算哈希值
        let hash = meta::data_hash(&data).map_err(log_error(handler_name))?;
        // 是否存在
        let exists_path: Option<(String,)> =
            sqlx::query_as("SELECT path FROM images WHERE hash=$1")
                .bind(&hash)
                .fetch_optional(&*state.pool)
                .await
                .map_err(Error::from)?;
        if let Some(exists_path) = exists_path {
            tracing::debug!("已存在：{}, {}", &hash, &exists_path.0);
            let url = format!("/{}", exists_path.0);
            return redirect(&url);
        }
        // 入库
        // 生成文件名
        let id = xid::new().to_string();
        let file_name = gen_filename(&file_name, &id);
        let dataline = chrono::Local::now();
        sqlx::query(
            r"INSERT INTO images (id, path, hash, dateline, is_deleted) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&file_name)
        .bind(&hash)
        .bind(&dataline)
        .bind(false)
        .execute(&*state.pool).await.map_err(Error::from)?;

        // 上传到对象存储
        let resp = (&*state.bucket)
            .put_object_with_content_type(&file_name, &data, &content_type)
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;

        tracing::debug!("已上传到对象存储：{:?}", resp);

        let url = format!("/{}", file_name);
        return redirect(&url);
    }

    Err(Error::from_str(crate::ErrorKind::Image, "未上传图片"))
}

fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {:?}", handler_name, err);
        err
    })
}

fn redirect(url: &str) -> Result<RedirectResponse> {
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, header, ()))
}

fn gen_filename(filename: &str, id: &str) -> String {
    meta::gen_filename(filename, Some(id))
}
