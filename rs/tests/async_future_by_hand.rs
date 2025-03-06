use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ManualFuture {
    inner: Arc<ManualInner>,
}

struct ManualInner {
    // Indicates whether the future is ready (completed).
    ready: AtomicBool,
    // Option<Waker> protected by a Mutex to store the pending waker.
    waker: Mutex<Option<Waker>>,
}

impl ManualFuture {
    pub fn new() -> Self {
        ManualFuture {
            inner: Arc::new(ManualInner {
                ready: AtomicBool::new(false),
                waker: Mutex::new(None),
            }),
        }
    }

    /// Triggers the completion of the future.
    /// Sets `ready` to true and wakes up the registered waker if it exists.
    pub fn trigger(&self) {
        self.inner.ready.store(true, Ordering::Release);
        if let Some(waker) = self.inner.waker.lock().unwrap().take() {
            waker.wake();
        }
    }
}

impl Future for ManualFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling...");
        // Quickly check if the future is already ready, without locking.
        if self.inner.ready.load(Ordering::Acquire) {
            return Poll::Ready(()); // should also clean the waker
        }

        // The future is not ready yet, so we register the waker to be woken up later.
        let mut waker_slot = self.inner.waker.lock().unwrap();

        // Double-check: Between the previous read and acquiring the mutex,
        // the status may have changed (the future might have become ready).
        if self.inner.ready.load(Ordering::Acquire) {
            return Poll::Ready(()); // should also clean the waker
        }

        // If no waker is registered or the current waker is different from the new one,
        // ALWAYS update with the current waker to avoid unnecessary wake-ups.
        let should_update = waker_slot
            .as_ref()
            .map(|w| !w.will_wake(cx.waker()))
            .unwrap_or(true);
        if should_update {
            *waker_slot = Some(cx.waker().clone());
        }
        Poll::Pending
    }
}

#[tokio::test]
async fn main() {
    let start = Instant::now();

    let manual_future = ManualFuture::new();

    // TODO What if we increase concurrent tasks
    for _ in 0..1 {
        // Simulate a trigger happening in another task
        let trigger_clone = manual_future.clone();
        tokio::spawn(async move {
            // Here, we simulate some waiting or computation...
            // For example, we can use a (non-blocking) sleep here.
            tokio::time::sleep(Duration::from_secs(1)).await;
            // std::thread::sleep(Duration::from_secs(1)); // TODO What if this sleep?
            trigger_clone.trigger();
            println!("Trigger called!");
        });
    }

    // The future will only complete once trigger() has been called.
    manual_future.await;
    println!("ManualFuture is complete!");

    let duration = Instant::now() - start;
    println!("Execution time: {duration:?}");
}
