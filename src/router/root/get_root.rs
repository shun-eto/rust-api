use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_root(Path(user_id): Path<String>) -> impl IntoResponse {
    println!("{:?}", user_id);
    (StatusCode::OK, Json(json!({"message":"get response!!"})))
}
