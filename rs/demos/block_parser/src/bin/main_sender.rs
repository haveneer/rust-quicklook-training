use axum::{extract::State, response::Json, routing::get, Router};
use block_parser::async_tools::{start_stats, Counters};
use block_parser::{make_test_transactions_with_seed, Block, OwnedBlock, ToBytes};
use chrono::Utc;
use serde_json::json;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::{Notify, RwLock};
use tokio::task::JoinHandle;
use tracing::{error, info, trace};

fn make_genesis_block() -> OwnedBlock {
    OwnedBlock::new(1, Utc::now().timestamp_millis() as u64, vec![], [0; 32])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger for debugging.
    console_subscriber::init();
    // tracing_subscriber::fmt::init();

    // Shared counter for the number of blocks sent.
    let sent_block_counter = Arc::new(AtomicUsize::new(0));
    let current_block = Arc::new(RwLock::new(None));
    let notify = Arc::new(Notify::new());

    let counters = Arc::new(Counters::new());
    let stats_handle = start_stats(counters.clone(), Duration::new(3, 0)).await;

    // Start the TCP server on port 9000.
    let tcp_manager = start_tcp_listener(
        sent_block_counter.clone(),
        current_block.clone(),
        notify.clone(),
    )
    .await;

    let http_server = start_http_service(sent_block_counter.clone(), current_block.clone()).await;

    let block_builder = start_block_builder(current_block.clone(), notify.clone(), counters.clone()).await;

    // Wait for both servers to run concurrently.
    tokio::try_join!(tcp_manager, http_server, block_builder, stats_handle)?;
    Ok(())
}

async fn start_tcp_listener(
    sent_block_counter: Arc<AtomicUsize>,
    current_block: Arc<RwLock<Option<OwnedBlock>>>,
    notify: Arc<Notify>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // Bind the TcpListener to port 9000 on all interfaces.
        let listener = TcpListener::bind("0.0.0.0:9000")
            .await
            .expect("Error binding to TCP port 9000");
        info!("TCP server listening on port 9000");

        loop {
            // Wait for an incoming connection.
            let (mut socket, addr) = listener.accept().await.expect("Error accepting connection");
            println!("Accepted connection from {:?}", addr);

            // Spawn an async task to handle the connection.
            let sent_block_counter = sent_block_counter.clone();
            let current_block = current_block.clone();
            let notify = notify.clone();
            tokio::spawn(async move {
                let mut current_block_id = 0;

                loop {
                    {
                        // Smaller scope to minimize lock
                        let current_block = current_block.read().await;
                        if let Some(current_block) = current_block.as_ref() {
                            if current_block.id() <= current_block_id {
                                info!("Wake up but already sent block");
                                continue;
                            }

                            // Send the entire sequence over the socket.
                            if let Err(e) = socket.write_all(&current_block.to_bytes()).await {
                                error!("Error sending data to {:?}: {:?}", addr, e);
                                break;
                            } else {
                                current_block_id = current_block.id();
                                trace!("Block {current_block_id} sent");
                                // Increment the counter of blocks sent.
                                sent_block_counter.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }

                    // Pause to avoid network saturation.
                    // tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                    notify.notified().await;
                }
            });
        }
    })
}

async fn start_http_service(
    sent_block_counter: Arc<AtomicUsize>,
    current_block: Arc<RwLock<Option<OwnedBlock>>>,
) -> JoinHandle<()> {
    // Define the HTTP handler using axum.
    async fn stats_handler(
        State((counter, current_block)): State<(Arc<AtomicUsize>, Arc<RwLock<Option<OwnedBlock>>>)>,
    ) -> Json<serde_json::Value> {
        let count = counter.load(Ordering::Relaxed);
        Json(json!({
            "blocks_sent": count,
            "current_block": current_block.read().await.as_ref().map_or(0, |b| b.id()),
        }))
    }

    // Create the axum router with the shared state.
    let app = Router::new()
        .route("/stats", get(stats_handler))
        .with_state((sent_block_counter, current_block));

    // Start the HTTP server on port 8080.
    tokio::spawn(async move {
        let addr = "0.0.0.0:8080".parse().unwrap();
        println!("HTTP server listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    })
}

fn hash_block_in_place(block: &OwnedBlock, hash: &mut [u8; 32]) {
    let mut hasher = Sha256::new();
    hasher.update(block.clone().to_bytes());
    hasher.finalize_into(hash.into());
}

async fn start_block_builder(
    current_block: Arc<RwLock<Option<OwnedBlock>>>,
    notify: Arc<Notify>,
    counters: Arc<Counters>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let genesis_block = make_genesis_block();
        let mut prev_hash = [0; 32];
        hash_block_in_place(&genesis_block, &mut prev_hash);
        let mut current_block_id = genesis_block.id();
        current_block.write().await.replace(genesis_block);

        loop {
            current_block_id += 1;
            let new_transactions = make_test_transactions_with_seed(current_block_id as u64);
            let new_block = OwnedBlock::new(
                current_block_id,
                Utc::now().timestamp_millis() as u64,
                new_transactions,
                prev_hash,
            );
            hash_block_in_place(&new_block, &mut prev_hash);
            trace!("New block {:?}", current_block_id);
            current_block.write().await.replace(new_block);
            counters.incr_index();
            // tokio::time::sleep(Duration::from_millis(1)).await;
            notify.notify_waiters();
        }
    })
}
