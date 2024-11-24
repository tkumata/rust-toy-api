use crate::application::convert_service;
use axum::{response::IntoResponse, response::Json};
use serde::Deserialize;

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

pub async fn convert_rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
    let converted: convert_service::ConvertedRgb =
        convert_service::to_hex(req_rgb.r, req_rgb.g, req_rgb.b);

    format!("#{}{}{}", converted.r, converted.g, converted.b)
}

pub async fn convert_bitv4(Json(req_prefix): Json<RequestBitV4>) -> impl IntoResponse {
    let converted = convert_service::to_subnetmask(req_prefix.bit_length);
    converted.to_string()
}
