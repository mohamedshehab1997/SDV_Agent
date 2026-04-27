use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await{
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            // Process the JSON value
            handle_request(request).await;
        }
    }
}
async fn handle_request(_request: Value) {
    // Here you can implement your logic to handle the JSON request
    println!("Received request: {}", _request);
}
