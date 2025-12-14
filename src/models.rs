use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub items: Vec<FileEntry>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfoResponse {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
    pub created: String,
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDirRequest {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteRequest {
    pub paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchMoveRequest {
    pub paths: Vec<String>,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCopyRequest {
    pub paths: Vec<String>,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        ApiResponse {
            code,
            message,
            data: None,
        }
    }
}
