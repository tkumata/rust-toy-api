use axum::{response::IntoResponse, response::Json};
use serde_json::json;

pub async fn healthcheck() -> impl IntoResponse {
    Json(json!("OK"))
}
