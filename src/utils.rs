use crate::error::{NoteApiFileError, NoteApiFileResult};
use crate::models::{AbsAndPath, CdResponse, ChangeInfo, FileEntry, LsResponse};
use std::path::PathBuf;
use std::{env, fs, io};
use axum::http::StatusCode;

pub async fn get_status_from_error(err: &NoteApiFileError, msg: &str) -> (StatusCode, Option<String>) {
    tracing::error!(error = ?err, msg);
    let status = match err {
        NoteApiFileError::FileNotFound(_) => StatusCode::NOT_FOUND,
        NoteApiFileError::IOError(e) => match e.kind() {
            io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    };
    (status, Some(err.to_string()))
}

pub async fn get_file_entry<P: AsRef<str>>(path: P) -> NoteApiFileResult<FileEntry> {
    let path = PathBuf::from(path.as_ref());
    let path = fs::canonicalize(path)?;

    let parent =  path.parent().unwrap();
    let entries = get_dir_entries(parent.display().to_string(), false).await?;

    Ok(entries.iter().find(|f| f.path.eq(&path.display().to_string())).unwrap().clone())
}

pub async fn get_dir_entries<P: AsRef<str>>(
    path: P,
    _all: bool,
) -> NoteApiFileResult<Vec<FileEntry>> {
    let path = PathBuf::from(path.as_ref());
    if !path.exists() {
        return Err(NoteApiFileError::FileNotFound(path.display().to_string()));
    }
    let read_dir = fs::read_dir(&path)?;

    let mut files = Vec::new();

    for entry in read_dir {
        let entry = entry?;

        let file_entry = FileEntry::try_from(entry)?;
        files.push(file_entry);
    }

    Ok(files)
}

pub async fn form_ls_response(
    path: String,
    entries: Vec<FileEntry>,
    error: Option<String>,
) -> LsResponse {
    // TODO: применить метод из handlers::api_ls
    let path = fs::canonicalize(&path)
        .map(|p| p.display().to_string())
        .unwrap_or(path);
    LsResponse {
        path,
        entries,
        error,
    }
}

pub async fn change_dir<P: AsRef<str>>(_from: P, to: P) -> NoteApiFileResult<ChangeInfo> {
    let from_dir = env::current_dir()?;
    let from_dir_abs = from_dir.canonicalize()?;
    
    let from_abs_and_path = AbsAndPath {
        path: from_dir.display().to_string(),
        abs_path: from_dir_abs.display().to_string(),
    };

    let to = to.as_ref();
    let to_path = PathBuf::from(to);
    let to_path_abs = to_path.canonicalize()?;
    let to_abs_and_path = AbsAndPath {
        path: to_path.display().to_string(),
        abs_path: to_path_abs.display().to_string(),
    };

    env::set_current_dir(to_path_abs)?;

    let new_current_dir = env::current_dir()?;

    Ok(ChangeInfo {
        from: from_abs_and_path,
        to: to_abs_and_path,
        new_current: new_current_dir.display().to_string(),
    })
}

// pub async fn form_cd_response(path: String, error: Option<String>) -> CdResponse {
//     match fs::canonicalize(&path) {
//         Ok(abs_path) => CdResponse {
//             path,
//             abs_path: abs_path.display().to_string(),
//             error,
//         },
//         Err(e) => {
//             let error = if let Some(error) = error {
//                 format!("{}\n {}", error, e)
//             } else {
//                 e.to_string()
//             };
//             CdResponse {
//                 path,
//                 abs_path: ".".to_string(),
//                 error: Some(error),
//             }
//         }
//     }
// }
