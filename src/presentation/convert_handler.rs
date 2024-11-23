use crate::application::convert_service;
use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestRgb {
    r: i32,
    g: i32,
    b: i32,
}

pub async fn convert_rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
    let converted: convert_service::ConvertedRgb =
        convert_service::to_hex(req_rgb.r, req_rgb.g, req_rgb.b);

    let rgb: String = format!("#{}{}{}", converted.r, converted.g, converted.b);

    (StatusCode::OK, rgb)
}

#[derive(Deserialize)]
pub struct RequestNetPrefix {
    bit_length: i32,
}

pub async fn convert_v4prefix(Json(req_prefix): Json<RequestNetPrefix>) -> impl IntoResponse {
    let converted: std::net::IpAddr = convert_service::to_subnetmask(req_prefix.bit_length);
    let subnetmask: String = converted.to_string();

    (StatusCode::OK, subnetmask)
}