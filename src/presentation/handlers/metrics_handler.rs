use axum::{response::IntoResponse, response::Json};
use serde_json::json;

use crate::interface::controllers::metrics_controller::MetricsController;

pub async fn get_metrics() -> impl IntoResponse {
    let controller = MetricsController::new();
    Json(json!(controller.get_metrics().await))
}

pub async fn get_kernel() -> impl IntoResponse {
    let controller = MetricsController::new();
    Json(json!(controller.get_kernel().await))
}

pub async fn get_cpu() -> impl IntoResponse {
    let controller = MetricsController::new();
    Json(json!(controller.get_cpu().await))
}

pub async fn get_memory() -> impl IntoResponse {
    let controller = MetricsController::new();
    Json(json!(controller.get_memory().await))
}

pub async fn get_storage() -> impl IntoResponse {
    let controller = MetricsController::new();
    Json(json!(controller.get_storage().await))
}
