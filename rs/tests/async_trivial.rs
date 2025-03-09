async fn async_example() {
    println!("async_example started");
    let operation = async_operation1();
    println!("operation1 created"); // no execution !
    let result = operation.await;
    println!("operation1 completed with result: {}", result);
    println!(
        "operation2 completed with result: {}",
        async_operation2().await
    );
    println!(
        "async closure completed with result: {}",
        async {
            println!("<message from async_closure>");
            String::from("result3")
        }
        .await
    );
    println!("async_example completed");
}

async fn async_operation1() -> String {
    println!("<message from async_operation1>");
    String::from("result1")
}

// async by hand
fn async_operation2() -> impl std::future::Future<Output = String> {
    // Or Box<dyn ...> is needed
    println!("<message from async_operation2>");
    std::future::ready(String::from("result2"))
}

#[tokio::main]
async fn main() {
    // need a runtime at top level or use futures::executor::block_on
    async_example().await
}

#[tokio::test]
async fn test() {
    async_example().await
}
