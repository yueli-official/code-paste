use axum::{ extract::{ Json, Path, Query, State }, response::IntoResponse, http::StatusCode };
use std::sync::Arc;

use sha2::{ Sha256, Digest };

use super::db::get_db;
use super::models::{
    CreateSnippetRequest,
    CreateSnippetReply,
    ErrorResponse,
    GetSnippetRequest,
    GetSnippetReply,
};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    // 转成16进制字符串
    hex::encode(result)
}

/// 验证密码
pub fn verify_password(password: &str, hashed: &str) -> bool {
    hash_password(password) == hashed
}

/// POST /snippets
pub async fn create_snippet_handler(Json(
    payload,
): Json<CreateSnippetRequest>) -> impl IntoResponse {
    let hashed_password = payload.password.as_ref().map(|p| hash_password(p));
    let snippet_payload = CreateSnippetRequest {
        password: hashed_password,
        ..payload
    };

    // snippet_payload.expires_in_seconds
    match get_db().create_snippet(snippet_payload) {
        Ok(snippet) =>
            (
                StatusCode::CREATED,
                Json(CreateSnippetReply {
                    success: true,
                    message: "Snippet created successfully".to_string(),
                    snippet: Some(snippet),
                }),
            ).into_response(),
        Err(err) =>
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateSnippetReply {
                    success: false,
                    message: format!("Failed to create snippet: {}", err),
                    snippet: None,
                }),
            ).into_response(),
    }
}

/// GET /snippets/{id}
pub async fn get_snippet_handler(
    Path(id): Path<String>,
    Query(params): Query<GetSnippetRequest>,
    State(_state): State<Arc<()>>
) -> impl IntoResponse {
    match get_db().get_snippet(&id) {
        Ok(Some(snippet)) => {
            if snippet.password_protected {
                if let Some(password) = params.password {
                    if !verify_password(&password, snippet.password.as_deref().unwrap_or("")) {
                        return (
                            StatusCode::UNAUTHORIZED,
                            Json(ErrorResponse {
                                error: "Unauthorized".to_string(),
                                message: "Incorrect password".to_string(),
                                code: 401,
                            }),
                        ).into_response();
                    }
                } else {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse {
                            code: 401,
                            error: "Unauthorized".to_string(),
                            message: "Password required".to_string(),
                        }),
                    ).into_response();
                }
            }

            let password_required = snippet.password_protected;
            (StatusCode::OK, Json(GetSnippetReply { snippet, password_required })).into_response()
        }
        Ok(None) =>
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    code: 404,
                    error: "NotFound".to_string(),
                    message: "Snippet not found".to_string(),
                }),
            ).into_response(),
        Err(err) =>
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    error: "DatabaseError".to_string(),
                    message: err.to_string(),
                }),
            ).into_response(),
    }
}
