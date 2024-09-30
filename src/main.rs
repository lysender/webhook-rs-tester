use reqwest::Client;
use serde::Serialize;

const APP_URL: &'static str = "http://localhost:9000/webhook";

#[derive(Serialize)]
struct CounterPayload {
    id: usize,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let handles = (0..100).map(|id| {
        let client_copy = client.clone();
        tokio::spawn(async move { send_request(client_copy, id).await })
    });

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn send_request(client: Client, id: usize) {
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
        println!("Request: {}, Response: {}", id, res_body);
    } else {
        println!("Request: {} failed.", id);
    }
}
