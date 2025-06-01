use crate::error::NoteApiFileError;
use crate::models::{CdRequest, CdResponse, FileRequest, FileResponse, LsRequest, LsResponse};
use crate::utils;
use axum::Json;
use axum::http::StatusCode;
use std::error::Error;
use std::io;

pub async fn api_get_file(
    Json(file_request): Json<FileRequest>,
) -> (StatusCode, Json<FileResponse>) {
    let result = utils::get_file_entry(&file_request.path).await;

    let (status, error_msg) = match &result {
        Err(e) => {
            utils::get_status_from_error(
                &e,
                format!("Failed to get file: {}", &file_request.path).as_str(),
            )
            .await
        }
        Ok(_) => (StatusCode::OK, None),
    };

    let file_entry = result.unwrap();

    (
        status,
        Json(FileResponse {
            file: file_entry,
            error: error_msg,
        }),
    )
}

pub async fn api_ls(Json(ls_request): Json<LsRequest>) -> (StatusCode, Json<LsResponse>) {
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
                },
            };

            (status, Some(e.to_string()))
        }
        Ok(_) => (StatusCode::OK, None),
    };
    let entries = result.unwrap_or_default();
    let response = utils::form_ls_response(ls_request.path, entries, error_msg);

    (status, Json(response.await))
}

pub async fn api_cd(Json(cd_request): Json<CdRequest>) -> (StatusCode, Json<CdResponse>) {
    let result = utils::change_dir(&cd_request.from, &cd_request.to).await;
    let (status, error_msg) = match &result {
        Err(e) => {
            utils::get_status_from_error(
                e,
                format!(
                    "Failed to change directory from: {}, to: {}",
                    &cd_request.from, &cd_request.to
                )
                .as_str(),
            )
            .await
        }
        Ok(_) => (StatusCode::OK, None),
    };

    let change_info = result.unwrap();

    (
        status,
        Json(CdResponse {
            from: change_info.from,
            to: change_info.to,
            error: error_msg,
        }),
    )
}
