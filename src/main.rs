use std::sync::Arc;

use axum::{ routing::{ post, get }, Router };

use crate::db::init_db;
use crate::router::{ create_snippet_handler, get_snippet_handler };
use tower_http::cors::{ CorsLayer, Any };
use tower_http::services::{ ServeDir, ServeFile };
use std::time::Duration;
use task::daily_task;

mod router;
mod models;
mod db;
mod task;
mod log;

#[tokio::main]
async fn main() {
    // 初始化数据库
    let _ = init_db();

    log::init_log();

    // 运行清理任务
    tokio::spawn(async {
        daily_task().await;
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    let app_state = Arc::new(());

    let serve_dir = ServeDir::new("frontend/dist").not_found_service(
        ServeFile::new("frontend/dist/index.html")
    );

    let app = Router::new()
        .route("/api/snippets", post(create_snippet_handler))
        .route("/api/snippets/{id}", get(get_snippet_handler))
        .fallback_service(serve_dir)
        .with_state(app_state)
        .layer(cors);

    tracing::info!("server listen on port 9000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
