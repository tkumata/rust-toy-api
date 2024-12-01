use crate::application::usecases::convert_service::ConvertService;
use axum::response::IntoResponse;

pub struct ConvertController;

impl ConvertController {
    pub fn new() -> Self {
        ConvertController
    }

    pub async fn convert_rgb(&self, r: i32, g: i32, b: i32) -> impl IntoResponse {
        let service = ConvertService::new();
        service.to_hex(r, g, b)
    }

    pub async fn convert_bitv4(&self, req_prefix: i32) -> impl IntoResponse {
        let service = ConvertService::new();
        service.to_subnetmask(req_prefix)
    }
}
