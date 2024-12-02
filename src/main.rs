mod application;
mod interface;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use presentation::handlers::convert_handler::ConvertHandler;
use presentation::handlers::generic_handler::GenericHandler;
use presentation::handlers::metrics_handler::MetricsHandler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(GenericHandler::healthcheck))
        // Return random number
        .route("/roll/1d6", get(GenericHandler::roll_1d6))
        // Sleep
        .route("/sleep/:wait_time", get(GenericHandler::make_sleep))
        // Get metrics.
        .route("/metrics", get(MetricsHandler::get_metrics))
        .route("/metrics/kernel", get(MetricsHandler::get_kernel))
        .route("/metrics/cpu", get(MetricsHandler::get_cpu))
        .route("/metrics/memory", get(MetricsHandler::get_memory))
        .route("/metrics/storage", get(MetricsHandler::get_storage))
        // Convert /27 to 255.255.255.224
        .route("/convert/bitv4", post(ConvertHandler::convert_bitv4))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(ConvertHandler::convert_rgb));

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
