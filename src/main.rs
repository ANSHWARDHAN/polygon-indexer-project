mod db;
mod indexer;
mod api;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Polygon POL net-flow indexer...");

    // Initialize DB
    db::init().expect("Failed to initialize DB");

    // Start indexer in background
    let handle = tokio::spawn(async {
        indexer::start().await;
    });

    // Start HTTP API (blocks current thread)
    api::run().await;

    // Wait for indexer if API exits
    let _ = handle.await;
}
