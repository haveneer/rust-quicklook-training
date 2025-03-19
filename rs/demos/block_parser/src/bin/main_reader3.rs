use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use block_parser::{Block, FromBytes, OwnedBlock, RefBlock};
use serde_json::json;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, RwLock};
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{debug, error, info, instrument, trace};

const BUFFER_SIZE: usize = 5 * 1024;

struct SharedBuffer {
    buffer: [u8; BUFFER_SIZE],
    read_offset: usize,
    write_offset: usize,
}

struct Counters {
    read: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize subscriber for profiling (e.g., using tokio-console).
    // console_subscriber::init();
    tracing_subscriber::fmt::init();

    // Connect to the server on port 9000.
    let stream = TcpStream::connect("127.0.0.1:9000").await?;
    println!("Connected to server on port 9000");

    // Shared state for the fixed buffer.
    let shared_buffer = Arc::new(Mutex::new(SharedBuffer {
        buffer: [0u8; BUFFER_SIZE],
        read_offset: 0,
        write_offset: 0,
    }));

    let counters = Arc::new(Mutex::new(Counters { read: 0 }));

    // Create a Notify to signal that new data is available.
    let notify = Arc::new(Notify::new());

    // Spawn the reader task to fill the buffer.
    let buffer_handle = start_filling_buffer(shared_buffer.clone(), notify.clone(), stream).await;

    // Spawn the processor task to process data from the buffer.
    let processor_handle =
        start_process_buffer(shared_buffer.clone(), counters.clone(), notify.clone()).await;

    // Spawn to display stats at a small frequency
    let stats_handle = start_stats(counters.clone()).await;

    // Wait for both tasks to complete.
    let _ = tokio::join!(buffer_handle, processor_handle, stats_handle);
    Ok(())
}

async fn start_stats(counters: Arc<Mutex<Counters>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        const PERIOD: u64 = 3000;

        let mut latest_read = counters.lock().await.read;
        let mut latest_time = Instant::now();
        loop {
            tokio::time::sleep(Duration::from_millis(PERIOD)).await;
            let current_read = counters.lock().await.read;
            let current_time = Instant::now();
            info!(
                "Total read: {current_read:8} [{:6.1} blocks/s]",
                (current_read - latest_read) as f64 / (current_time - latest_time).as_secs_f64()
            );
            latest_read = current_read;
            latest_time = current_time;
        }
    })
}

async fn start_process_buffer(
    shared_buffer: Arc<Mutex<SharedBuffer>>,
    counters: Arc<Mutex<Counters>>,
    notify: Arc<Notify>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            // Wait until notified that new data is available.
            notify.notified().await;
            loop {
                let mut buffer_guard = shared_buffer.lock().await;
                let available_data =
                    &buffer_guard.buffer[buffer_guard.read_offset..buffer_guard.write_offset];
                let consumed = process_buffer(available_data).await.unwrap_or(0);
                if consumed > 0 {
                    counters.lock().await.read += 1;
                    buffer_guard.read_offset += consumed;
                } else {
                    break;
                }
            }
        }
    })
}

async fn start_filling_buffer(
    shared_buffer: Arc<Mutex<SharedBuffer>>,
    notify: Arc<Notify>,
    mut stream: TcpStream,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            // Lock the buffer to determine free space.
            let mut buffer_guard = shared_buffer.lock().await;
            if buffer_guard.write_offset >= BUFFER_SIZE {
                // If the buffer is full, compact it.
                let range = buffer_guard.read_offset..buffer_guard.write_offset;
                buffer_guard.buffer.copy_within(range, 0);
                buffer_guard.write_offset -= buffer_guard.read_offset;
                buffer_guard.read_offset = 0;
            }
            // Create a mutable slice for writing new data.
            let free_space = buffer_guard.write_offset..BUFFER_SIZE;
            let read_slice = &mut buffer_guard.buffer[free_space];

            // Use read (not read_exact) to avoid blocking for a full slice.
            let n = match stream.read(read_slice).await {
                Ok(0) => {
                    info!("Connection closed by the server.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    error!("Error reading from stream: {:?}", e);
                    break;
                }
            };
            buffer_guard.write_offset += n;
            debug!(
                "Reader: Received {} bytes, write_offset now {}",
                n, buffer_guard.write_offset
            );
            drop(buffer_guard);

            tokio::time::sleep(std::time::Duration::from_millis(20)).await;

            // Simulate an asynchronous operation.
            tokio::task::yield_now().await;

            // Notify the processor that new data is available.
            notify.notify_waiters();
        }
    })
}

/// An asynchronous processing function that simulates an operation "f":
/// - Randomly selects a number `n` (between 40 and 1000) of bytes to process.
/// - If the data slice has at least `n` bytes, it awaits a simulated asynchronous operation,
///   computes the sum of the first `n` bytes, prints the result, and returns `n`
///   to indicate the number of bytes consumed.
/// - Otherwise, it returns 0.
#[instrument(level = "info", skip_all)]
async fn process_buffer(data: &[u8]) -> Result<usize, Box<dyn Error>> {
    let (block, offset) = RefBlock::from_bytes(data)?;
    trace!("Block: {}", block.id());

    // tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // Simulate an asynchronous operation.
    tokio::task::yield_now().await;

    Ok(offset)
}
