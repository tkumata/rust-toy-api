use axum::{response::IntoResponse, response::Json};
use serde::Deserialize;

use crate::interface::controllers::convert_controller::ConvertController;

#[derive(Deserialize)]
pub struct RequestRgb {
    r: i32,
    g: i32,
    b: i32,
}

#[derive(Deserialize)]
pub struct RequestBitV4 {
    bit_length: i32,
}

pub struct ConvertHandler;

impl ConvertHandler {
    pub async fn convert_rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
        let controller = ConvertController::new();
        controller
            .convert_rgb(req_rgb.r, req_rgb.g, req_rgb.b)
            .await
    }

    pub async fn convert_bitv4(Json(req_prefix): Json<RequestBitV4>) -> impl IntoResponse {
        let controller = ConvertController::new();
        controller.convert_bitv4(req_prefix.bit_length).await
    }
}
