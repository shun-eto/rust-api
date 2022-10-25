use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_inquiries() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message":"get inquiries!!"})))
}
