use std::net::IpAddr;

pub struct ConvertService;

pub struct ConvertedRgb {
    pub r: String,
    pub g: String,
    pub b: String,
}

fn calc_hex(value: i32) -> String {
    let value: i32 = value.clamp(0, 255);
    format!("{:02X}", value)
}

impl ConvertService {
    pub fn new() -> Self {
        ConvertService
    }

    pub fn to_hex(&self, r: i32, g: i32, b: i32) -> ConvertedRgb {
        ConvertedRgb {
            r: calc_hex(r).to_string(),
            g: calc_hex(g).to_string(),
            b: calc_hex(b).to_string(),
        }
    }

    pub fn to_subnetmask(&self, bit_length: i32) -> IpAddr {
        let bit: u32 = (!0) << (32 - bit_length);
        IpAddr::V4(bit.into())
    }
}
