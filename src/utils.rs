use crate::error::{NoteApiFileError, NoteApiFileResult};
use crate::models::{AbsAndPath, CdResponse, FileEntry, LsResponse};
use std::path::PathBuf;
use std::{env, fs};

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
    match fs::canonicalize(&path) {
        Ok(path) => {
            LsResponse {
                path: path.display().to_string(),
                entries,
                error
            }
        },
        Err(e) => {
            let error = if let Some(error) = error {
                format!("{}\n {}", error, e)
            } else {
                e.to_string()
            };
            LsResponse {
                path,
                entries,
                error: Some(error)
            }
        }
    }
}

pub async fn change_dir<P: AsRef<str>>(_from: P, to: P) -> NoteApiFileResult<AbsAndPath> {
    let _from_dir = env::current_dir()?;
    let to = to.as_ref();

    let to_path = PathBuf::from(to);
    if !to_path.exists() && !to_path.is_dir() {
        return Err(NoteApiFileError::FileNotFound(to.to_string()));
    }

    env::set_current_dir(to)?;

    Ok(AbsAndPath {
        path: to.to_string(),
        abs_path: fs::canonicalize(to.to_string())?.display().to_string(),
    })
}

pub async fn form_cd_response(path: String, error: Option<String>) -> CdResponse {
    match fs::canonicalize(&path) {
        Ok(abs_path) => {
            CdResponse {
                path,
                abs_path: abs_path.display().to_string(),
                error
            }
        }
        Err(e) => {
            let error = if let Some(error) = error {
                format!("{}\n {}", error, e)
            } else {
                e.to_string()
            };
            CdResponse {
                path,
                abs_path: ".".to_string(),
                error: Some(error)
            }
        }
    }
}