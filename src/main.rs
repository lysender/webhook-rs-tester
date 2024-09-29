use serde::Serialize;
use tracing::{error, info};

mod error;

pub use error::{Error, Result};

const APP_URL: &'static str = "http://localhost:4200/webhook";

#[derive(Serialize)]
struct CounterPayload {
    id: usize,
}

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "webhook_rs_tester=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let handles = (0..20).map(|id| tokio::spawn(async move { send_request(id).await }));

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn send_request(id: usize) {
    let client = reqwest::Client::new();
    let data = CounterPayload { id };
    let post_body = serde_json::to_string(&data).unwrap();
    let response = client
        .post(APP_URL)
        .header("Content-Type", "application/json")
        .body(post_body)
        .send()
        .await
        .unwrap();
    if response.status().is_success() {
        let res_body = response.text().await.unwrap();
        info!("Request: {}, Response: {}", id, res_body);
    } else {
        error!("Request: {} failed.", id);
    }
}
