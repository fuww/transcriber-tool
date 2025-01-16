mod transcription;

use axum::{Router, Server};
use std::sync::Arc;
use std::net::SocketAddr;

use transcription::{TranscriptionService, routes::transcription_routes};

#[tokio::main]
async fn main() {
    // Load API key from environment
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set");

    // Initialize the transcription service
    let transcription_service = Arc::new(TranscriptionService::new(api_key));

    // Build our application with routes
    let app = Router::new()
        .merge(transcription_routes(transcription_service));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}