use axum::{response::IntoResponse, response::Json};
use serde::Serialize;
use std::net::IpAddr;

#[derive(Serialize)]
pub struct ConvertedRgb {
    pub hex: String,
}

#[derive(Serialize)]
pub struct NetMask {
    pub mask: IpAddr,
}

fn calc_hex(value: i32) -> String {
    let value: i32 = value.clamp(0, 255);
    format!("{:02X}", value)
}

pub struct ConvertService;

impl ConvertService {
    pub fn new() -> Self {
        ConvertService
    }

    pub fn to_hex(&self, r: i32, g: i32, b: i32) -> impl IntoResponse {
        let converted = ConvertedRgb {
            hex: format!("#{}{}{}", calc_hex(r), calc_hex(g), calc_hex(b)),
        };
        Json(converted)
    }

    pub fn to_subnetmask(&self, bit_length: i32) -> impl IntoResponse {
        let bit: u32 = (!0) << (32 - bit_length);
        let netmask = NetMask {
            mask: IpAddr::V4(bit.into()),
        };
        Json(netmask)
    }
}
