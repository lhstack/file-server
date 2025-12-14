use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use crate::models::FileEntry;

pub fn sanitize_path(path: &str) -> String {
    path.trim_start_matches('/')
        .trim_end_matches('/')
        .to_string()
}

pub fn is_safe_path(base: &Path, target: &Path) -> bool {
    // 规范化路径而不要求它们存在
    let base_normalized = normalize_path(base);
    let target_normalized = normalize_path(target);
    
    target_normalized.starts_with(&base_normalized)
}

fn normalize_path(path: &Path) -> PathBuf {
    use std::path::Component;
    
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                normalized.pop();
            }
            Component::CurDir => {
                // 忽略当前目录
            }
            _ => {
                normalized.push(component);
            }
        }
    }
    normalized
}

pub async fn create_file_entry(path: &Path, base_dir: &Path) -> anyhow::Result<FileEntry> {
    let metadata = tokio::fs::metadata(path).await?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let relative_path = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace("\\", "/");

    let modified = metadata.modified()?;
    let created = metadata.created()?;

    let modified_str = format_system_time(modified);
    let created_str = format_system_time(created);

    Ok(FileEntry {
        name,
        path: relative_path.to_string(),
        is_dir: metadata.is_dir(),
        size: metadata.len(),
        modified: modified_str,
        created: created_str,
    })
}

pub fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_mime_type(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| {
            Some(match ext {
                "txt" => "text/plain",
                "html" => "text/html",
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "pdf" => "application/pdf",
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                "mp4" => "video/mp4",
                "mp3" => "audio/mpeg",
                "zip" => "application/zip",
                _ => return None,
            }.to_string())
        })
}
