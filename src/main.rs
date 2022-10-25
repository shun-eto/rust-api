use axum::response::IntoResponse;
use axum::Json;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use dotenv::dotenv;
use rust_api::util::error_handler::handle_timeout_error;
use serde_json::json;
use std::time::Duration;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async {})).layer(
        ServiceBuilder::new()
            // `timeout` will produce an error if the handler takes
            // too long so we must handle those
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(1)),
    );

    let address = format!("0.0.0.0:{port}", port = 3000);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_root() -> impl IntoResponse {
    println!("{}", "error message");
    panic!("test");
    (StatusCode::OK, Json(json!({"message":"post response!!"})))
}
async fn post_root() -> impl IntoResponse {
    panic!("error");
    (StatusCode::OK, Json(json!({"message":"post response!!"})))
}
async fn delete_root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message":"delete response!!"})))
}
async fn patch_root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message":"patch response!!"})))
}

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}
