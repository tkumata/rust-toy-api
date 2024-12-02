use axum::{extract::Path, response::IntoResponse, response::Json};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct Dice {
    dice: i32,
}

#[derive(Deserialize, Serialize)]
pub struct WaitDurationPath {
    wait_time: i32,
}

pub struct GenericHandler;

impl GenericHandler {
    pub async fn healthcheck() -> impl IntoResponse {
        Json(json!("OK"))
    }

    pub async fn roll_1d6() -> impl IntoResponse {
        let mut rnd: rand::rngs::ThreadRng = rand::thread_rng();
        let i: i32 = rnd.gen_range(1..6);
        let dice: Dice = Dice { dice: i };
        Json(json!(dice))
    }

    pub async fn make_sleep(Path(path_param): Path<WaitDurationPath>) -> impl IntoResponse {
        let d = path_param.wait_time as u64;
        sleep(Duration::from_secs(d)).await; // threading 中なので tokio の sleep を利用する。
        Json(json!(d))
    }
}
