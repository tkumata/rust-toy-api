mod application;
mod models;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use presentation::convert_handler;
use presentation::dice_handler;
use presentation::healthcheck_handler;
use presentation::metrics_handler;
use presentation::sleep_handler;

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
        // Convert /27 to 255.255.255.224
        .route("/convert/v4prefix", post(convert_handler::convert_v4prefix))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(convert_handler::convert_rgb));

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
