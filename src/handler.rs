use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{AppState, Result};

pub async fn upload_ui(State(state): State<Arc<AppState>>) -> Result<Html<String>> {
    unimplemented!()
}

pub async fn result_ui(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Result<Html<String>> {
    unimplemented!()
}

pub async fn upload(State(state): State<Arc<AppState>>) -> Result<()> {
    unimplemented!()
}
