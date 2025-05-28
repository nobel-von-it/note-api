use std::fs;
use std::path::PathBuf;
use crate::error::{NoteApiLsError, NoteApiLsResult, NoteApiResult};
use crate::models::FileEntry;

pub async fn get_dir_entries<P: AsRef<str>>(path: P, all: bool) -> NoteApiLsResult<Vec<FileEntry>> {
    let path = PathBuf::from(path.as_ref());
    if !path.exists() {
        return Err(NoteApiLsError::FileNotFound(path.display().to_string()));
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