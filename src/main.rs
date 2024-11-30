mod application;
mod interface;

use axum::{
    routing::{get, post},
    Router,
};
use interface::controllers::convert_controller;
use interface::controllers::dice_controller;
use interface::controllers::healthcheck_controller;
use interface::controllers::metrics_controller;
use interface::controllers::sleep_controller;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(healthcheck_controller::healthcheck))
        // Return random number
        .route("/roll/1d6", get(dice_controller::roll_1d6))
        // Sleep
        .route("/sleep/:wait_time", get(sleep_controller::make_sleep))
        // Get metrics.
        .route("/metrics", get(metrics_controller::get_metrics))
        .route("/metrics/cpuload", get(metrics_controller::get_cpuload))
        .route("/metrics/memusage", get(metrics_controller::get_memusage))
        .route("/metrics/diskusage", get(metrics_controller::get_diskusage))
        // Convert /27 to 255.255.255.224
        .route("/convert/bitv4", post(convert_controller::convert_bitv4))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(convert_controller::convert_rgb));

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
