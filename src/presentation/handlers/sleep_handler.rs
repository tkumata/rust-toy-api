use axum::{extract::Path, response::IntoResponse, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize)]
pub struct WaitDurationPath {
    wait_time: i32,
}

pub async fn make_sleep(Path(path_param): Path<WaitDurationPath>) -> impl IntoResponse {
    let d = path_param.wait_time as u64;
    sleep(Duration::from_secs(d)).await; // threading 中なので tokio の sleep を利用する。

    Json(json!(d))
}
