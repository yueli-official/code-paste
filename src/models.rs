use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub id: String,
    pub title: String,
    pub language: String,
    pub content: String,
    pub description: String,
    pub password_protected: bool,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub view_count: i32,
    pub tags: Option<String>,
    pub visibility: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippetRequest {
    pub title: String,
    pub language: String,
    pub content: String,
    pub description: Option<String>,
    pub password: Option<String>,
    pub expires_in_seconds: Option<i32>,
    pub author: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnippetReply {
    pub success: bool,
    pub message: String,
    pub snippet: Option<Snippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSnippetRequest {
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSnippetReply {
    pub snippet: Snippet,
    pub password_required: bool,
}
