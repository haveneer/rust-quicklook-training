use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::info;

pub struct Counters {
    index: AtomicUsize,
}

impl Counters {
    pub fn new() -> Self {
        Self {
            index: AtomicUsize::new(0),
        }
    }

    pub fn incr_index(&self) {
        self.index.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_index(&self) -> usize {
        self.index.load(Ordering::Relaxed)
    }
}

pub async fn start_stats(counters: Arc<Counters>, period: Duration) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut latest_index = counters.get_index();
        let mut latest_time = Instant::now();
        loop {
            tokio::time::sleep(period).await;
            let current_index = counters.get_index();
            let current_time = Instant::now();
            info!(
                "Current index: {current_index:8} [{:6.1} blocks/s]",
                (current_index - latest_index) as f64 / (current_time - latest_time).as_secs_f64()
            );
            latest_index = current_index;
            latest_time = current_time;
        }
    })
}
