use axum::{
    extract::{Path, Multipart, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncReadExt;

use crate::{
    models::*,
    utils::*,
    errors::AppError,
    AppState,
};

use tracing;

// 列表文件（根目录）
pub async fn list_files_root(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<FileListResponse>>, AppError> {
    list_files_impl(state, "").await
}

// 列表文件（带路径）
pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<ApiResponse<FileListResponse>>, AppError> {
    list_files_impl(state, &path).await
}

// 列表文件实现
async fn list_files_impl(
    state: Arc<AppState>,
    path: &str,
) -> Result<Json<ApiResponse<FileListResponse>>, AppError> {
    let safe_path = sanitize_path(path);
    
    let full_path = if safe_path.is_empty() {
        state.base_dir.clone()
    } else {
        state.base_dir.join(&safe_path)
    };

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("Path not found".to_string()));
    }

    if !full_path.is_dir() {
        return Err(AppError::InvalidPath("Not a directory".to_string()));
    }

    let mut entries = Vec::new();
    let mut read_dir = fs::read_dir(&full_path).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let entry_path = entry.path();
        if let Ok(file_entry) = create_file_entry(&entry_path, &state.base_dir).await {
            entries.push(file_entry);
        }
    }

    // 排序：文件夹在前，按名称排序
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });

    let total = entries.len();
    let response = FileListResponse { items: entries, total };
    
    Ok(Json(ApiResponse::success(response)))
}

// 文件预览
pub async fn preview_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe_path = sanitize_path(&path);
    let full_path = state.base_dir.join(&safe_path);

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("File not found".to_string()));
    }

    if full_path.is_dir() {
        return Err(AppError::InvalidPath("Cannot preview directory".to_string()));
    }

    let content = fs::read(&full_path).await?;
    Ok((StatusCode::OK, content))
}

// 文件上传（根目录）
pub async fn upload_file_root(
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    upload_file_impl(state, "", multipart).await
}

// 文件上传（带路径）
pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    upload_file_impl(state, &path, multipart).await
}

// 文件上传实现
async fn upload_file_impl(
    state: Arc<AppState>,
    path: &str,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let safe_path = sanitize_path(path);
    
    let target_dir = if safe_path.is_empty() {
        state.base_dir.clone()
    } else {
        state.base_dir.join(&safe_path)
    };

    if !is_safe_path(&state.base_dir, &target_dir) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    fs::create_dir_all(&target_dir).await?;

    let mut uploaded_files = Vec::new();
    let mut error_count = 0;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::InvalidRequest(format!("Multipart error: {}", e))
    })? {
        // 跳过没有文件名的字段
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue,
        };

        match field.bytes().await {
            Ok(data) => {
                let file_path = target_dir.join(&file_name);
                match fs::write(&file_path, data).await {
                    Ok(_) => {
                        uploaded_files.push(file_name);
                    }
                    Err(e) => {
                        tracing::error!("Failed to write file {}: {}", file_name, e);
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to read field {}: {}", file_name, e);
                error_count += 1;
            }
        }
    }

    if uploaded_files.is_empty() && error_count > 0 {
        return Err(AppError::InvalidRequest(format!(
            "Failed to upload {} files",
            error_count
        )));
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "uploaded": uploaded_files
    }))))
}

// 删除文件
pub async fn delete_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe_path = sanitize_path(&path);
    let full_path = state.base_dir.join(&safe_path);

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("File not found".to_string()));
    }

    if full_path.is_dir() {
        return Err(AppError::InvalidPath("Use delete-dir for directories".to_string()));
    }

    fs::remove_file(&full_path).await?;
    Ok(Json(ApiResponse::<()>::success(())))
}

// 删除文件夹
pub async fn delete_directory(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe_path = sanitize_path(&path);
    let full_path = state.base_dir.join(&safe_path);

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("Directory not found".to_string()));
    }

    if !full_path.is_dir() {
        return Err(AppError::InvalidPath("Not a directory".to_string()));
    }

    fs::remove_dir_all(&full_path).await?;
    Ok(Json(ApiResponse::<()>::success(())))
}

// 创建文件夹
pub async fn create_directory(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateDirRequest>,
) -> Result<impl IntoResponse, AppError> {
    let safe_path = sanitize_path(&req.path);
    let parent_dir = if safe_path.is_empty() {
        state.base_dir.clone()
    } else {
        state.base_dir.join(&safe_path)
    };

    if !is_safe_path(&state.base_dir, &parent_dir) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    let new_dir = parent_dir.join(&req.name);

    if !is_safe_path(&state.base_dir, &new_dir) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if new_dir.exists() {
        return Err(AppError::InvalidRequest("Directory already exists".to_string()));
    }

    fs::create_dir_all(&new_dir).await?;
    Ok(Json(ApiResponse::<()>::success(())))
}

// 获取文件信息
pub async fn get_file_info(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe_path = sanitize_path(&path);
    let full_path = state.base_dir.join(&safe_path);

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("File not found".to_string()));
    }

    let metadata = fs::metadata(&full_path).await?;
    let name = full_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let relative_path = full_path
        .strip_prefix(&state.base_dir)
        .unwrap_or(&full_path)
        .to_string_lossy()
        .replace("\\", "/");

    let modified = format_system_time(metadata.modified()?);
    let created = format_system_time(metadata.created()?);
    let mime_type = get_mime_type(&full_path);

    let info = FileInfoResponse {
        name,
        path: relative_path.to_string(),
        is_dir: metadata.is_dir(),
        size: metadata.len(),
        modified,
        created,
        mime_type,
    };

    Ok(Json(ApiResponse::success(info)))
}

// 批量删除
pub async fn batch_delete(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchDeleteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut deleted = Vec::new();
    let mut failed = Vec::new();

    for path in req.paths {
        let safe_path = sanitize_path(&path);
        let full_path = state.base_dir.join(&safe_path);

        if !is_safe_path(&state.base_dir, &full_path) {
            failed.push((path, "Access denied".to_string()));
            continue;
        }

        if !full_path.exists() {
            failed.push((path, "Not found".to_string()));
            continue;
        }

        match if full_path.is_dir() {
            fs::remove_dir_all(&full_path).await
        } else {
            fs::remove_file(&full_path).await
        } {
            Ok(_) => deleted.push(path),
            Err(e) => failed.push((path, e.to_string())),
        }
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "deleted": deleted,
        "failed": failed
    }))))
}

// 批量移动
pub async fn batch_move(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchMoveRequest>,
) -> Result<impl IntoResponse, AppError> {
    let dest_safe = sanitize_path(&req.destination);
    let dest_dir = state.base_dir.join(&dest_safe);

    if !is_safe_path(&state.base_dir, &dest_dir) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    fs::create_dir_all(&dest_dir).await?;

    let mut moved = Vec::new();
    let mut failed = Vec::new();

    for path in req.paths {
        let safe_path = sanitize_path(&path);
        let full_path = state.base_dir.join(&safe_path);

        if !is_safe_path(&state.base_dir, &full_path) {
            failed.push((path, "Access denied".to_string()));
            continue;
        }

        if !full_path.exists() {
            failed.push((path, "Not found".to_string()));
            continue;
        }

        let file_name = full_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let new_path = dest_dir.join(file_name);

        match fs::rename(&full_path, &new_path).await {
            Ok(_) => moved.push(path),
            Err(e) => failed.push((path, e.to_string())),
        }
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "moved": moved,
        "failed": failed
    }))))
}

// 批量复制
pub async fn batch_copy(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchCopyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let dest_safe = sanitize_path(&req.destination);
    let dest_dir = state.base_dir.join(&dest_safe);

    if !is_safe_path(&state.base_dir, &dest_dir) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    fs::create_dir_all(&dest_dir).await?;

    let mut copied = Vec::new();
    let mut failed = Vec::new();

    for path in req.paths {
        let safe_path = sanitize_path(&path);
        let full_path = state.base_dir.join(&safe_path);

        if !is_safe_path(&state.base_dir, &full_path) {
            failed.push((path, "Access denied".to_string()));
            continue;
        }

        if !full_path.exists() {
            failed.push((path, "Not found".to_string()));
            continue;
        }

        let file_name = full_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let new_path = dest_dir.join(file_name);

        match if full_path.is_dir() {
            copy_dir_recursive(full_path.clone(), new_path).await
        } else {
            fs::copy(&full_path, &new_path).await.map(|_| ())
        } {
            Ok(_) => copied.push(path),
            Err(e) => failed.push((path, e.to_string())),
        }
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "copied": copied,
        "failed": failed
    }))))
}

fn copy_dir_recursive(
    src: std::path::PathBuf,
    dst: std::path::PathBuf,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
    Box::pin(async move {
        fs::create_dir_all(&dst).await?;
        let mut entries = fs::read_dir(&src).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = entry.file_name();
            let new_path = dst.join(&file_name);

            if path.is_dir() {
                copy_dir_recursive(path, new_path).await?;
            } else {
                fs::copy(&path, &new_path).await?;
            }
        }

        Ok(())
    })
}

// 文件下载（支持Range请求用于seek）
pub async fn download_file(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    headers: axum::http::HeaderMap,
) -> Result<Response, AppError> {
    let safe_path = sanitize_path(&path);
    let full_path = state.base_dir.join(&safe_path);

    if !is_safe_path(&state.base_dir, &full_path) {
        return Err(AppError::PermissionDenied("Access denied".to_string()));
    }

    if !full_path.exists() {
        return Err(AppError::NotFound("File not found".to_string()));
    }

    if full_path.is_dir() {
        return Err(AppError::InvalidPath("Cannot download directory".to_string()));
    }

    let file_name = full_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download");

    let file_size = fs::metadata(&full_path).await?.len();
    let mime_type = get_mime_type(&full_path)
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // 检查Range请求头
    if let Some(range_header) = headers.get(header::RANGE) {
        if let Ok(range_str) = range_header.to_str() {
            if let Some(range) = parse_range_header(range_str, file_size) {
                let (start, end) = range;
                let content_length = end - start + 1;
                
                let file = tokio::fs::File::open(&full_path).await?;
                let mut reader = tokio::io::BufReader::new(file);
                
                // 跳转到指定位置
                use tokio::io::AsyncSeekExt;
                reader.seek(std::io::SeekFrom::Start(start)).await?;
                
                let body = axum::body::Body::from_stream(
                    tokio_util::io::ReaderStream::new(reader.take(content_length))
                );

                return Ok((
                    StatusCode::PARTIAL_CONTENT,
                    [
                        (header::CONTENT_TYPE, mime_type),
                        (header::CONTENT_LENGTH, content_length.to_string()),
                        (
                            header::CONTENT_RANGE,
                            format!("bytes {}-{}/{}", start, end, file_size),
                        ),
                        (header::ACCEPT_RANGES, "bytes".to_string()),
                    ],
                    body,
                )
                    .into_response());
            }
        }
    }

    // 普通请求，返回完整文件
    let content = fs::read(&full_path).await?;

    Ok((
        [
            (header::CONTENT_TYPE, mime_type),
            (header::CONTENT_LENGTH, content.len().to_string()),
            (header::ACCEPT_RANGES, "bytes".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{}\"", file_name),
            ),
        ],
        content,
    )
        .into_response())
}

// 解析Range请求头
fn parse_range_header(range_str: &str, file_size: u64) -> Option<(u64, u64)> {
    if !range_str.starts_with("bytes=") {
        return None;
    }

    let range_part = &range_str[6..];
    
    if let Some(dash_pos) = range_part.find('-') {
        let start_str = &range_part[..dash_pos];
        let end_str = &range_part[dash_pos + 1..];

        let start = start_str.parse::<u64>().ok()?;
        let end = if end_str.is_empty() {
            file_size - 1
        } else {
            end_str.parse::<u64>().ok()?
        };

        if start <= end && end < file_size {
            return Some((start, end));
        }
    }

    None
}

// 前端页面处理器
pub async fn index() -> ([(header::HeaderName, &'static str); 1], &'static str) {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../static/index.html"),
    )
}

pub async fn style() -> ([(header::HeaderName, &'static str); 1], &'static str) {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../static/style.css"),
    )
}

pub async fn app_js() -> ([(header::HeaderName, &'static str); 1], &'static str) {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        include_str!("../static/app.js"),
    )
}
