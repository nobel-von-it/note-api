use crate::models::{LsRequest, LsResponse};
use crate::utils;
use axum::http::StatusCode;
use axum::Json;

pub async fn api_ls(Json(ls_request): Json<LsRequest>) -> (StatusCode, Json<LsResponse>) {
    match utils::get_dir_entries(&ls_request.path, ls_request.all).await {
        Ok(entries) => (
            StatusCode::OK,
            Json(LsResponse {
                path: ls_request.path,
                entries,
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LsResponse {
                path: ls_request.path,
                entries: vec![],
                error: Some(err.to_string()),
            }),
        ),
    }
}
