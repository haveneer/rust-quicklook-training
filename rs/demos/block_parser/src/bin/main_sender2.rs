use axum::{extract::State, response::Json, routing::get, Router};
use block_parser::async_tools::{start_stats, Counters};
use block_parser::{IncrementalBlockBuilder, ParseError, Transaction};
use chrono::Utc;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
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
use tracing::{error, info};

const MAX_BLOCK_SIZE: usize = 1024;

struct BuiltBlocks {
    raw_data: Vec<u8>,
    len: usize,
    count: usize,
}

impl BuiltBlocks {
    fn data(&self) -> &[u8] {
        &self.raw_data[..self.len]
    }

    fn reset(&mut self) {
        self.len = 0;
        self.count = 0;
    }
}

impl BuiltBlocks {
    fn new() -> BuiltBlocks {
        BuiltBlocks {
            raw_data: vec![0; MAX_BLOCK_SIZE],
            len: 0,
            count: 0,
        }
    }
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

    let block_builder =
        start_block_builder(current_block.clone(), counters.clone(), notify.clone()).await;

    // Wait for both servers to run concurrently.
    tokio::try_join!(tcp_manager, http_server, block_builder, stats_handle)?;
    Ok(())
}

async fn start_tcp_listener(
    sent_block_counter: Arc<AtomicUsize>,
    current_block: Arc<RwLock<Option<BuiltBlocks>>>,
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
                        if let Some(built_blocks) = current_block.as_ref() {
                            // FIXME
                            // if current_block.id() <= current_block_id {
                            //     info!("Wake up but already sent block");
                            //     continue;
                            // }

                            // Send the entire sequence over the socket.
                            if let Err(e) = socket.write_all(built_blocks.data()).await {
                                error!("Error sending data to {:?}: {:?}", addr, e);
                                break;
                            } else {
                                // Increment the counter of blocks sent.
                                sent_block_counter.fetch_add(built_blocks.count, Ordering::Relaxed);
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
    current_block: Arc<RwLock<Option<BuiltBlocks>>>,
) -> JoinHandle<()> {
    // Define the HTTP handler using axum.
    async fn stats_handler(
        State((counter, current_block)): State<(
            Arc<AtomicUsize>,
            Arc<RwLock<Option<BuiltBlocks>>>,
        )>,
    ) -> Json<serde_json::Value> {
        let count = counter.load(Ordering::Relaxed);
        // let current_block = if let Some((data, len)) = current_block.read().await.as_ref() {
        //     let (block, verified_len) = RefBlock::from_bytes(&data[0..*len]).unwrap();
        //     assert_eq!(*len, verified_len);
        //     block.id()
        // } else {
        //     0
        // };

        Json(json!({
            "blocks_sent": count,
            "current_block": 0,
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

fn hash_block_in_place(data: &[u8], hash: &mut [u8; 32]) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize_into(hash.into());
}

fn append_new_block(
    current_block_id: u32,
    prev_hash: &[u8; 32],
    transaction_count: usize,
    multi_blocks_data: &mut [u8],
) -> Result<usize, ParseError> {
    let mut new_block = IncrementalBlockBuilder::new(multi_blocks_data)?;
    let mut new_block = new_block.with_id(current_block_id)?;
    let mut new_block = new_block.with_timestamp(Utc::now().timestamp_millis() as u64)?;

    let mut r = StdRng::seed_from_u64(current_block_id as u64);
    for _ in 0..transaction_count {
        const MAX_TRANSACTION_SIZE: usize = 100;
        let mut transaction_data = [0u8; MAX_TRANSACTION_SIZE];
        let n = r.gen_range(0..MAX_TRANSACTION_SIZE);
        transaction_data[..n].iter_mut().for_each(|x| *x = r.gen());
        new_block.add_transaction(&mut transaction_data)?;
    }

    let (_, data_len) = new_block.with_prev_hash(&prev_hash)?;
    Ok(data_len)
}

async fn process_one_new_block(
    current_block_id: &mut u32,
    prev_hash: &mut [u8; 32],
    transaction_count: usize,
    counters: Arc<Counters>,
    current_block: Arc<RwLock<Option<BuiltBlocks>>>,
    notify: Arc<Notify>,
    mut multi_blocks_data: BuiltBlocks,
) -> BuiltBlocks {
    if let Ok(block_size) = append_new_block(
        *current_block_id,
        &prev_hash,
        transaction_count,
        &mut multi_blocks_data.raw_data[multi_blocks_data.len..],
    ) {
        hash_block_in_place(
            &multi_blocks_data.raw_data[multi_blocks_data.len..multi_blocks_data.len + block_size],
            prev_hash,
        );
        multi_blocks_data.len += block_size;
        multi_blocks_data.count += 1;
        counters.incr_index();
        *current_block_id += 1;
    } else {
        multi_blocks_data = current_block
            .write()
            .await
            .replace(multi_blocks_data)
            .or_else(|| Some(BuiltBlocks::new()))
            .unwrap();
        multi_blocks_data.reset();
        notify.notify_waiters();
    }
    multi_blocks_data
}

async fn start_block_builder(
    current_block: Arc<RwLock<Option<BuiltBlocks>>>,
    counters: Arc<Counters>,
    notify: Arc<Notify>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut multi_blocks_data = BuiltBlocks::new();
        let mut prev_hash = [0; 32];
        let mut current_block_id = 1;

        multi_blocks_data = process_one_new_block(
            &mut current_block_id,
            &mut prev_hash,
            0,
            counters.clone(),
            current_block.clone(),
            notify.clone(),
            multi_blocks_data,
        )
        .await;

        loop {
            multi_blocks_data = process_one_new_block(
                &mut current_block_id,
                &mut prev_hash,
                0,
                counters.clone(),
                current_block.clone(),
                notify.clone(),
                multi_blocks_data,
            )
            .await;
            // tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
}
