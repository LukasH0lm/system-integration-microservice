use infrastructure::messaging::MessageDispatcher;
use std::sync::{Arc, Mutex};

mod api;
mod domain;
mod infrastructure;
mod shared;

#[tokio::main]
async fn main() {
    println!("Booting up the microservice...");

    // =================================================================
    // STEP 4: BACKGROUND SERVICE (RabbitMQ Simulation)
    // =================================================================
    let mut dispatcher = MessageDispatcher::new();

    // Register the handler for our domain event
    dispatcher.register("channel.created", |payload| async move {
        let msg = String::from_utf8_lossy(&payload);
        println!(
            "--> [RabbitMQ Task] Received event: 'channel.created' | Data: {}",
            msg
        );
    });

    // Spawn the background worker to run concurrently with the API
    tokio::spawn(async move {
        println!("--> Background Message Service started...");
        loop {
            let dummy_payload = b"Syncing new channel from microservice B".to_vec();
            dispatcher.dispatch("channel.created", dummy_payload).await;

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    // =================================================================
    // REST API SETUP
    // =================================================================
    // 1. Initialize our Dependency Injection container
    let state = Arc::new(Mutex::new(Vec::new()));

    // 2. Build the router and inject the state
    let app = api::router::build_router(state);

    // 3. Bind to a port and start the server on the main thread
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("--> Server is live and listening on http://localhost:3000");

    // This blocks the main thread, keeping the app alive to serve web requests
    axum::serve(listener, app).await.unwrap();
}
