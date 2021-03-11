#[cfg(test)]
mod tests {
    const N: usize = 50;

    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test() {
        // Spawn a few threads to increment a shared variable (non-atomically), and
        // let the main thread know once all increments are done.
        //
        // Here we're using an Arc to share memory among threads, and the data inside
        // the Arc is protected with a mutex.
        let data = Arc::new(Mutex::new(0));

        let (tx, rx) = channel::<bool>();
        for i in 0..N {
            let (data, tx) = (Arc::clone(&data), tx.clone());
            thread::spawn(move || {
                // The shared state can only be accessed once the lock is held.
                // Our non-atomic increment is safe because we're the only thread
                // which can access the shared state when the lock is held.
                //
                // We unwrap() the return value to assert that we are not expecting
                // threads to ever fail while holding the lock.
                let mut data = data.lock().unwrap();
                *data += 1;
                if *data == N || i % 10 == 0 {
                    println!("Process {} sends update", i);
                    tx.send(*data == N).unwrap();
                }
                // the lock is unlocked here when `data` goes out of scope.
            });
        }

        while !rx.recv().unwrap() {
            println!("In progress... data = {}", data.lock().unwrap());
        }
        println!("Final data = {}", data.lock().unwrap());
    }
}
