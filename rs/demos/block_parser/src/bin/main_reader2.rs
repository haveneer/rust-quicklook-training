use block_parser::async_tools::{start_stats, Counters};
use block_parser::{Block, FromBytes, RefBlock};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::join;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tracing::trace;

const BUFFER_SIZE: usize = 50 * 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the subscriber for profiling (e.g., with tokio-console).
    // console_subscriber::init();
    tracing_subscriber::fmt::init();

    // Connect to the server on port 9000.
    let stream = TcpStream::connect("127.0.0.1:9000").await?;
    println!("Connected to server on port 9000");

    // Convert the TCP stream into an asynchronous stream of byte chunks.
    let mut stream = ReaderStream::new(stream);

    let mut buffer = vec![0; BUFFER_SIZE];
    // 'read_offset' tracks the beginning of the data to process.
    let mut read_offset: usize = 0;
    // 'write_offset' tracks the end of the data in the buffer.
    let mut write_offset: usize = 0;

    let counters = Arc::new(Mutex::new(Counters::new()));

    // Spawn to display stats at a small frequency
    let stats_handle = start_stats(counters.clone(), Duration::new(3, 0)).await;

    // Process each chunk from the async stream.
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        let chunk_len = chunk.len();

        // If adding the new chunk would overflow the buffer, compact the buffer.
        if write_offset + chunk_len >= BUFFER_SIZE {
            eprintln!("Buffer full, performing compaction.");
            buffer.copy_within(read_offset..write_offset, 0);
            write_offset -= read_offset;
            read_offset = 0;
            if write_offset + chunk_len >= BUFFER_SIZE {
                eprintln!("Chunk too large even after compaction, skipping chunk.");
                continue;
            }
        }

        // Copy the new chunk into the buffer at the current write_offset.
        buffer[write_offset..write_offset + chunk_len].copy_from_slice(&chunk);
        write_offset += chunk_len;
        eprintln!(
            "Received chunk of {} bytes, write_offset now {}",
            chunk_len, write_offset
        );

        // Process the data in the buffer as long as possible.
        while {
            let consumed = process_buffer(&buffer[read_offset..write_offset])
                .await
                .unwrap_or(0);
            if consumed == 0 {
                // Not enough data to process a full block.
                buffer.copy_within(read_offset..write_offset, 0);
                write_offset -= read_offset;
                read_offset = 0;
                false
            } else {
                read_offset += consumed;
                counters.lock().await.incr_index();
                true
            }
        } {}

        eprintln!("Remaining data in buffer: {}", write_offset);
    }

    join!(stats_handle);

    Ok(())
}

/// An asynchronous processing:
/// returns Err if insufficient data
#[tracing::instrument(level = "info", skip_all)]
async fn process_buffer(data: &[u8]) -> Result<usize, Box<dyn Error>> {
    let (block, offset) = RefBlock::from_bytes(data)?;
    trace!("Block: {}", block.id());

    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // Simulate an asynchronous operation.
    tokio::task::yield_now().await;

    Ok(offset)
}
