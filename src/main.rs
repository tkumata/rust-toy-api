mod application;
mod interface;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use interface::controllers::convert_controller;
use presentation::handlers::dice_handler;
use presentation::handlers::healthcheck_handler;
use presentation::handlers::metrics_handler;
use presentation::handlers::sleep_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(healthcheck_handler::healthcheck))
        // Return random number
        .route("/roll/1d6", get(dice_handler::roll_1d6))
        // Sleep
        .route("/sleep/:wait_time", get(sleep_handler::make_sleep))
        // Get metrics.
        .route("/metrics", get(metrics_handler::get_metrics))
        .route("/metrics/kernel", get(metrics_handler::get_kernel))
        .route("/metrics/cpu", get(metrics_handler::get_cpu))
        .route("/metrics/memory", get(metrics_handler::get_memory))
        .route("/metrics/storage", get(metrics_handler::get_storage))
        // Convert /27 to 255.255.255.224
        .route("/convert/bitv4", post(convert_controller::convert_bitv4))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(convert_controller::convert_rgb));

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
