use std::error::Error;
use std::io;
use crate::models::{CdRequest, CdResponse, LsRequest, LsResponse};
use crate::utils;
use axum::Json;
use axum::http::StatusCode;
use crate::error::NoteApiFileError;

pub async fn api_ls(
    Json(ls_request): Json<LsRequest>,
) -> (StatusCode, Json<LsResponse>) {
    // match utils::get_dir_entries(&ls_request.path, ls_request.all).await {
    //     Ok(entries) => (
    //         StatusCode::OK,
    //         Json(utils::form_ls_response(ls_request.path, entries, None).await),
    //     ),
    //     Err(err) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(utils::form_ls_response(ls_request.path, vec![], Some(err.to_string())).await),
    //     ),
    // }
    let result = utils::get_dir_entries(&ls_request.path, ls_request.all).await;
    
    let (status, error_msg) = match &result {
        Err(e) => {
            tracing::error!(error = ?e, "Failed to read directory '{}'", &ls_request.path);
            let status = match e {
                NoteApiFileError::FileNotFound(_) => StatusCode::NOT_FOUND,
                NoteApiFileError::IOError(io_e) => match io_e.kind() {
                    io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
            };

            (status, Some(e.to_string()))
        }
        Ok(_) => (StatusCode::OK, None),
    };
    let entries = result.unwrap_or_default();
    let response = utils::form_ls_response(
        ls_request.path,
        entries,
        error_msg,
    );

    (status, Json(response.await))
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
