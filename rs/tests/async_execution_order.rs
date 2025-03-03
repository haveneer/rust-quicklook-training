use tokio::time::{sleep, Duration};

#[tokio::test] // Proc-macro to define the Tokio runtime
async fn main() {
    // Start the first asynchronous task
    let task1 = tokio::spawn(async {
        println!("Task1: starting");
        sleep(Duration::from_secs(2)).await; // waits 2s without blocking the thread
        // TODO what about the following alternative?
        // std::thread::sleep(Duration::from_secs(2));
        println!("Task1: finished after 2s");
    });

    // Start a second asynchronous task
    let task2 = tokio::spawn(async {
        println!("Task2: starting");
        sleep(Duration::from_secs(1)).await; // waits 1s
        println!("Task2: finished after 1s");
    });

    // Wait for both tasks to finish
    task1.await.unwrap();
    println!("Task1 awaited");
    task2.await.unwrap();
    println!("Task2 awaited");
    println!("All tasks are completed");
}
