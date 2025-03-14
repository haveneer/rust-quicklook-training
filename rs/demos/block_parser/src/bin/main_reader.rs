use block_parser::{Block, FromBytes, RefBlock};
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::info;

const BUFFER_SIZE: usize = 50 * 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // console_subscriber::init();
    tracing_subscriber::fmt::init();

    // let log_file = std::fs::File::create("log.log")?;
    // tracing_subscriber::fmt()
    //     // all spans/events with a level higher than DEBUG (e.g, info, warn, etc.)
    //     // will be written to stdout.
    //     .with_max_level(tracing::Level::DEBUG)
    //     // Events will be synthesized whenever a span is created, entered, exited, or closed.
    //     // The close event will contain the span's busy and idle time if timestamps are enabled.
    //     .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    //     // sets this to be the default, global subscriber for this application.
    //     .with_writer(std::sync::Mutex::new(log_file))
    //     .init();

    // Connect to the server on port 9000.
    let mut stream = TcpStream::connect("127.0.0.1:9000").await?;
    println!("Connected to server on port 9000");

    let mut buffer = [0u8; BUFFER_SIZE];
    // 'offset' tracks the position of reader / writer in the buffer
    let mut read_offset: usize = 0;
    let mut write_offset: usize = 0;

    loop {
        // Instrumentation: Create a span for the read operation.
        let read_span = tracing::span!(tracing::Level::INFO, "read_from_stream");
        let _read_enter = read_span.enter();

        // Read data from the stream into the buffer starting at 'offset'.
        let n = stream.read_exact(&mut buffer[write_offset..]).await?;
        if n == 0 {
            println!("Connection closed by the server.");
            break;
        }
        write_offset += n;
        assert!(write_offset <= BUFFER_SIZE);

        // Process the data in the buffer as long as possible.
        while {
            assert!(read_offset <= write_offset);
            let consumed = process_buffer(&buffer[read_offset..]).await.unwrap_or(0);
            if consumed == 0 {
                // Not enough data to process a full segment.
                buffer.copy_within(read_offset..write_offset, 0);
                write_offset -= read_offset;
                read_offset = 0;
                false
            } else {
                read_offset += consumed;
                true
            }
        } {}

        eprintln!("Remaining data in buffer: {write_offset}");
    }

    Ok(())
}

#[tracing::instrument(level = "info", skip_all, ret)]
async fn process_buffer(data: &[u8]) -> Result<usize, Box<dyn Error>> {
    let (block, offset) = RefBlock::from_bytes(data)?;
    info!("Block: {}", block.id());

    // tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    Ok(offset)

    // let mut rng = rand::thread_rng();
    // let n = rng.gen_range(40..=1000);
    // if data.len() < n {
    //     println!("Failed to read {n} < {}", data.len());
    //     return 0;
    // }
    //
    // // Fake async yield
    // tokio::task::yield_now().await;
    //
    // let sum: u32 = data[..n].iter().map(|&b| b as u32).sum();
    // println!("Processed {} bytes, sum = {}", n, sum);
    // n
}
