use pin_project::pin_project;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncReadExt, ReadBuf};

const BUFFER_SIZE: usize = 2048;

/// The enumeration representing the states of our state machine.
/// Each variant stores the data it needs.
///
/// pin_project macro will help to simply manage pinned fields required
/// by self-referential structures like many state machines generated for async functions.
///
/// The pin_project crate automates the process of safely “projecting” a pinned reference
/// of the parent structure into pinned references of its fields.
/// For example, calling self.project() gives you safe access to the fields
/// with their proper pinning guarantees (as a result, the #[pin] annotated fields are wrapped
/// in a Pin<&mut T> automatically). This removes manual 'unsafe' code sections.
#[pin_project(project = StateProj)]
enum State {
    /// In the Opening state, we store the file path and the future to open the file.
    Opening {
        file_path: PathBuf,
        #[pin]
        open_future: Pin<Box<dyn Future<Output = io::Result<File>> + Send>>,
    },
    /// In the Reading state, the file is opened and we have a buffer to read chunks.
    Reading {
        #[pin]
        file: File,
        buffer: [u8; BUFFER_SIZE],
        hasher: DefaultHasher,
    },
    /// The terminal state.
    Done,
}

/// The main structure contains only one field: the current state.
#[pin_project]
pub struct ReadFileByChunksFuture {
    #[pin]
    state: State,
}

impl ReadFileByChunksFuture {
    /// Creates a new future that will read the file located at `path` in chunks.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        // Start in the Opening state: prepare the future to open the file.
        let open_future = Box::pin(File::open(path.clone()));
        Self {
            state: State::Opening {
                file_path: path,
                open_future,
            },
        }
    }
}

impl Future for ReadFileByChunksFuture {
    type Output = io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Project the structure to access the state without borrow conflicts.
        let mut this = self.project();
        loop {
            match this.state.as_mut().project() {
                StateProj::Opening {
                    file_path: _,
                    open_future,
                } => {
                    // Poll the future for opening the file.
                    match open_future.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(result) => {
                            match result {
                                Ok(file) => {
                                    // Transition to the Reading state.
                                    // Initialize a new buffer.
                                    *this.state = State::Reading {
                                        file,
                                        buffer: [0u8; BUFFER_SIZE],
                                        hasher: Default::default(),
                                    };
                                    // Continue the loop to process the Reading state.
                                    continue;
                                }
                                Err(e) => {
                                    *this.state = State::Done;
                                    return Poll::Ready(Err(e));
                                }
                            }
                        }
                    }
                }
                StateProj::Reading {
                    file,
                    buffer,
                    hasher,
                } => {
                    // Prepare a ReadBuf from the buffer.
                    let mut read_buf = ReadBuf::new(buffer);
                    match file.poll_read(cx, &mut read_buf) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Ok(())) => {
                            let n = read_buf.filled().len();
                            if n == 0 {
                                println!("Final hash: {:x}", hasher.finish());
                                // End of file reached.
                                *this.state = State::Done;
                                return Poll::Ready(Ok(()));
                            }
                            read_buf.filled().hash(hasher);
                            println!("Read {} bytes", n);
                            // Continue reading (remain in the Reading state).
                            continue;
                        }
                        Poll::Ready(Err(e)) => {
                            *this.state = State::Done;
                            return Poll::Ready(Err(e));
                        }
                    }
                }
                StateProj::Done => return Poll::Ready(Ok(())),
            }
        }
    }
}

pub fn read_file_desugared(path: &str) -> ReadFileByChunksFuture {
    ReadFileByChunksFuture::new(path)
}

async fn read_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path).await?;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut hasher = DefaultHasher::new();

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        buffer[..n].hash(&mut hasher);
        println!("Read {} bytes", n);
    }
    println!("Final hash: {:x}", hasher.finish());
    Ok(())
}

#[tokio::test]
async fn main() -> io::Result<()> {
    let path = file!();

    println!("Reading with async/await:");
    read_file(path).await?;

    println!("\nReading with manual Future:");
    read_file_desugared(path).await?;

    Ok(())
}
