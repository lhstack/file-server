use axum::{
    routing::{get, post, delete},
    Router,
    extract::DefaultBodyLimit,
};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::fs;
use tower_http::cors::{CorsLayer, Any};

mod handlers;
mod models;
mod utils;
mod errors;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 读取配置，如果不存在则使用默认值
    let (base_dir, host, port) = match std::fs::read_to_string("config.json") {
        Ok(config_str) => {
            match serde_json::from_str::<serde_json::Value>(&config_str) {
                Ok(config) => {
                    let dir = config["dir"]
                        .as_str()
                        .unwrap_or("./public")
                        .to_string();
                    let h = config["host"]
                        .as_str()
                        .unwrap_or("127.0.0.1")
                        .to_string();
                    let p = config["port"]
                        .as_str()
                        .unwrap_or("8080")
                        .to_string();
                    (dir, h, p)
                }
                Err(_) => {
                    eprintln!("警告: config.json格式错误，使用默认配置");
                    ("./public".to_string(), "127.0.0.1".to_string(), "8080".to_string())
                }
            }
        }
        Err(_) => {
            eprintln!("警告: 未找到config.json，使用默认配置");
            ("./public".to_string(), "127.0.0.1".to_string(), "8080".to_string())
        }
    };

    // 创建基础目录
    fs::create_dir_all(&base_dir).await?;

    let state = AppState {
        base_dir: PathBuf::from(base_dir),
    };

    // 构建路由
    let app = Router::new()
        // 前端页面
        .route("/", get(handlers::index))
        .route("/index.html", get(handlers::index))
        .route("/style.css", get(handlers::style))
        .route("/app.js", get(handlers::app_js))
        // API路由 - 更具体的路由放在前面
        .route("/api/batch-delete", post(batch_delete))
        .route("/api/batch-move", post(batch_move))
        .route("/api/batch-copy", post(batch_copy))
        .route("/api/mkdir", post(create_directory))
        .route("/api/files", get(list_files_root))
        .route("/api/upload", post(upload_file_root))
        // 文件列表
        .route("/api/files/{*path}", get(list_files))
        // 文件预览
        .route("/api/preview/{*path}", get(preview_file))
        // 文件下载
        .route("/api/download/{*path}", get(handlers::download_file))
        // 文件上传
        .route("/api/upload/{*path}", post(upload_file))
        // 文件删除
        .route("/api/delete/{*path}", delete(delete_file))
        // 文件夹删除
        .route("/api/delete-dir/{*path}", delete(delete_directory))
        // 文件信息
        .route("/api/info/{*path}", get(get_file_info))
        // 增加body大小限制到1GB
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .layer(
            CorsLayer::permissive()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    tracing::info!("Server running on http://{}:{}", host, port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
