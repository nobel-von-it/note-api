use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{DirEntry, FileType};

#[derive(Deserialize)]
pub struct FileRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct FileResponse {
    pub file: FileEntry,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct LsRequest {
    pub path: String,
    pub depth: usize,
    pub all: bool,
}

#[derive(Serialize)]
pub struct LsResponse {
    pub path: String,
    pub entries: Vec<FileEntry>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,

    pub file_type: FType,
    pub size: usize,
    pub content: Option<Vec<String>>,

    // timestamp
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
impl TryFrom<DirEntry> for FileEntry {
    type Error = std::io::Error;
    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let name = entry.file_name().to_str().unwrap().to_string();
        let path = fs::canonicalize(entry.path())?
            .to_str()
            .unwrap()
            .to_string();

        let file_type = FType::from(entry.file_type()?);

        let content: Option<Vec<String>> = match file_type {
            FType::File => {
                let file = fs::read_to_string(entry.path())?;
                Some(file.lines().map(|l| l.to_string()).collect())
            }
            _ => None,
        };
        let metadata = entry.metadata()?;
        let size = metadata.len() as usize;

        let created_at: DateTime<Local> = metadata.created()?.into();
        let updated_at: DateTime<Local> = metadata.modified()?.into();

        Ok(FileEntry {
            name,
            path,
            content,
            file_type,
            size,
            created_at,
            updated_at,
        })
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum FType {
    Dir,
    File,
    Link,
    Other,
}
// impl<S: AsRef<str>> From<S> for FType {
//     fn from(value: S) -> Self {
//         let s = value.as_ref();
//         match s {
//             "dir" => FType::Dir,
//             "file" => FType::File,
//             "link" => FType::Link,
//             _ => FType::Other,
//         }
//     }
// }
impl From<FileType> for FType {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            FType::Dir
        } else if value.is_file() {
            FType::File
        } else if value.is_symlink() {
            FType::Link
        } else {
            FType::Other
        }
    }
}

#[derive(Deserialize)]
pub struct CdRequest {
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct AbsAndPath {
    pub path: String,
    pub abs_path: String,
}

#[derive(Serialize)]
pub struct CdResponse {
    pub from: AbsAndPath,
    pub to: AbsAndPath,
    pub error: Option<String>,
}

pub struct ChangeInfo {
    pub from: AbsAndPath,
    pub to: AbsAndPath,
    pub new_current: String,
}
