#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    fn do_expensive_job(id: i8) {
        for _ in 0..10 {
            println!("Thread {} [{:?}] working...", id, thread::current().id());
            thread::sleep(Duration::from_millis(100));
        }
        // if id == 2 {
        //     panic!()
        // }
    }

    #[test]
    fn do_two_jobs() {
        // Spawn two _expensive_ threads to do work.
        let thread_one = thread::spawn(|| do_expensive_job(1));
        let thread_two = thread::spawn(|| do_expensive_job(2));

        // Wait for both threads to complete.
        thread_one.join().expect("thread one panicked");
        thread_two.join().expect("thread two panicked");
    }
}
