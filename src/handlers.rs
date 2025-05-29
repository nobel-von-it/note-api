use crate::models::{CdRequest, CdResponse, LsRequest, LsResponse};
use crate::utils;
use axum::Json;
use axum::http::StatusCode;

pub async fn api_ls(Json(ls_request): Json<LsRequest>) -> (StatusCode, Json<LsResponse>) {
    match utils::get_dir_entries(&ls_request.path, ls_request.all).await {
        Ok(entries) => (
            StatusCode::OK,
            Json(utils::form_ls_response(ls_request.path, entries, None).await),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(utils::form_ls_response(ls_request.path, vec![], Some(err.to_string())).await),
        ),
    }
}

pub async fn api_cd(Json(cd_request): Json<CdRequest>) -> (StatusCode, Json<CdResponse>) {
    match utils::change_dir(cd_request.from, cd_request.to.clone()).await {
        Ok(new_dir) => (
            StatusCode::OK,
            Json(utils::form_cd_response(new_dir.path, None).await),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(utils::form_cd_response(cd_request.to, Some(err.to_string())).await),
        ),
    }
}
