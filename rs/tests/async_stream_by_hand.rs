use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::stream::Stream;

/// A manual Stream that allows injecting elements from the outside.
pub struct ManualStream<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

struct Inner<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    closed: bool,
}

impl<T> ManualStream<T> {
    /// Creates a new ManualStream, empty and open.
    pub fn new() -> Self {
        ManualStream {
            inner: Arc::new(Mutex::new(Inner {
                queue: VecDeque::new(),
                waker: None,
                closed: false,
            })),
        }
    }

    /// Adds an element to the stream and wakes the consumer if necessary.
    pub fn push(&self, item: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.queue.push_back(item);
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }

    /// Closes the stream. Once closed, no new element will be accepted,
    /// and the consumer will receive `None` once the queue is emptied.
    pub fn close(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.closed = true;
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

// To allow sharing the stream (for example, between producer and consumer)
impl<T> Clone for ManualStream<T> {
    fn clone(&self) -> Self {
        ManualStream {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Stream for ManualStream<T> {
    type Item = T;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut inner = self.inner.lock().unwrap();
        // If there is an element in the queue, return it immediately.
        if let Some(item) = inner.queue.pop_front() {
            return Poll::Ready(Some(item));
        }
        // If the queue is empty and the stream is closed, the stream is finished.
        if inner.closed {
            return Poll::Ready(None);
        }
        // Otherwise, register the waker to be woken when an element is pushed.
        inner.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

use futures::stream::StreamExt; // To use .next() on the stream.
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn main() {
    let stream = ManualStream::new();

    // Cloning so that elements can be pushed from another task.
    let producer = stream.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            // Simulate a delay or a computation before adding the element.
            sleep(Duration::from_millis(200)).await;
            println!("Producing element {}", i);
            producer.push(i);
        }
        // Close the stream to indicate that no more elements will be produced.
        producer.close();
    });

    tokio::pin!(stream);
    // - Without `tokio::pin!`: For an `Unpin` type, a simple mutable variable is sufficient.
    // - With `tokio::pin!`: You ensure that even if the type is not `Unpin`, it will be correctly pinned,
    //   making your code more robust and explicit about the pinning requirements for asynchronous APIs.

    // Consumer that retrieves elements as soon as they are available.
    while let Some(item) = stream.next().await {
        println!("Consuming element {}", item);
    }
    println!("Stream finished.");
}
