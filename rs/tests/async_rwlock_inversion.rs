use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::{Duration, Instant};

const UNIT: Duration = Duration::from_millis(100);
const SLEEP: Duration = Duration::from_millis(300);

fn spawn_reader(
    lock: &Arc<RwLock<i32>>,
    id: u32,
    start: Instant,
    pending_writer: &Arc<RwLock<bool>>,
    should_stop: &Arc<RwLock<bool>>,
) -> thread::JoinHandle<()> {
    let reader_lock = Arc::clone(lock);
    let pending_writer = Arc::clone(pending_writer);
    let should_stop = Arc::clone(should_stop);
    thread::spawn(move || {
        thread::sleep(id * UNIT);

        println!(
            "Reader {id} is trying to acquire the lock at {:?}",
            start.elapsed()
        );

        // Readers acquire and hold the lock for a long time
        let _guard = reader_lock.read().unwrap();
        println!("Reader {} acquired the lock at {:?}", id, start.elapsed());
        if *pending_writer.read().unwrap() {
            eprintln!("** Reader acquires lock while Writer was already waiting for");
            *should_stop.write().unwrap() = true;
        }

        // Simulate long work
        thread::sleep(SLEEP);

        println!("Reader {} releases the lock at {:?}", id, start.elapsed());
    })
}

fn spawn_writer(
    lock: &Arc<RwLock<i32>>,
    delay: Duration,
    start: Instant,
    pending_writer: &Arc<RwLock<bool>>,
) -> thread::JoinHandle<()> {
    let writer_lock = lock.clone();
    let pending_writer = Arc::clone(pending_writer);
    thread::spawn(move || {
        thread::sleep(delay);

        // Launch the high-priority writer, which now has to wait
        println!(
            "Writer is trying to acquire the lock at {:?}",
            start.elapsed()
        );
        *pending_writer.write().unwrap() = true;

        // The writer attempts to acquire the lock but must wait for all readers
        let mut guard = writer_lock.write().unwrap();
        println!("Writer acquired the lock at {:?}", start.elapsed());

        *pending_writer.write().unwrap() = false;

        // Modify the value
        *guard += 1;

        println!("Writer finished at {:?}", start.elapsed());
    })
}

fn main() {
    for _ in 0..100 {
        println!("=============================================");
        // Capture the start time to measure durations
        let start = Instant::now();
        // Inversion priority issue detection
        let pending_writer = Arc::new(RwLock::new(false));
        let should_stop = Arc::new(RwLock::new(false));
        let mut handles = Vec::new();

        // Create a shared RwLock
        let lock = Arc::new(RwLock::new(0));

        // Start multiple "low-priority" readers and one "high priority" writer
        handles.push(spawn_writer(&lock, UNIT, start, &pending_writer)); // waits for 2 UNITs before locking
        handles.push(spawn_reader(&lock, 0, start, &pending_writer, &should_stop));
        handles.push(spawn_reader(&lock, 1, start, &pending_writer, &should_stop));
        handles.push(spawn_reader(&lock, 2, start, &pending_writer, &should_stop));
        handles.push(spawn_reader(&lock, 3, start, &pending_writer, &should_stop));

        for jh in handles.into_iter() {
            jh.join().unwrap();
        }

        if *should_stop.read().unwrap() {
            break;
        }
    }
}

#[test]
fn test() {
    main()
}
