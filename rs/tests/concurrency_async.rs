#[cfg(test)]

/// cf [async book](https://rust-lang.github.io/async-book/)
///
/// **Futures are inert** in Rust and make progress only when polled. Dropping a future stops it from making further progress.
///
/// **Async is zero-cost** in Rust, which means that you only pay for what you use. Specifically, you can use async without heap allocations and dynamic dispatch, which is great for performance! This also lets you use async in constrained environments, such as embedded systems.
///
/// **No built-in runtime** is provided by Rust. Instead, runtimes are provided by community maintained crates.
///
/// **Both single- and multithreaded** runtimes are available in Rust, which have different strengths and weaknesses.
mod tests {
    use futures::executor::block_on;
    use futures::join;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    async fn do_expensive_job_async(id: i8) {
        for _ in 0..10 {
            println!("Thread {} working...", id);
            thread::sleep(Duration::from_millis(100));
        }
        // if id == 2 {
        //     panic!()
        // }
    }

    #[test]
    fn do_three_jobs() {
        // requires to leave out async world using block_on like functions
        block_on(async {
            // Spawn two _expensive_ threads to do work.
            let thread_one = do_expensive_job_async(1);
            let thread_two = do_expensive_job_async(2);
            let thread_three = do_expensive_job_async(3);
            thread_three.await;
            // Wait for both to complete.
            join!(thread_one, thread_two); // no await/join => never run
        });
    }

    struct SharedData {
        data: u64,
    }

    async fn do_exclusive_job_async(_id: i8, shared_data: &Mutex<&mut SharedData>) {
        let mut data = shared_data.lock().unwrap(); // unwrap without no guard: we are sure to be not poisoned
                                                    // example of poisoning: the other 'task' panics while it has locked the resource.
        data.data += 1;
    }

    #[test]
    fn test_shared_data() {
        let mut data = SharedData { data: 0 };

        block_on(async {
            let data = Mutex::new(&mut data);
            let t1 = do_exclusive_job_async(1, &data);
            let t2 = do_exclusive_job_async(2, &data);
            // Wait for both to complete.
            join!(t1, t2);
        });

        println!("Final data is {}", data.data);
    }

    #[test]
    fn test_async_return_type() {
        // By default, async returns an impl Future<ReturnType>
        // It could be automatically computed for univoque types
        let _a = async { 1 };

        struct MyError;

        // But for Option, Result... an instance of a variant of the type doesn't provide the complete type
        // let _b = async { Ok(()) }; // error: cannot infer type for type parameter `E` declared on the enum `Result`

        // To solve this, you have to provide type annonciations
        let _c = async { Ok::<i32, MyError>(1) }; // using complete returned type
    }
}
