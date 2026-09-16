use std::sync::mpsc;
use std::thread;

fn main() {
    let data: Vec<f64> = (1..=1000).map(f64::from).collect();
    let (tx, rx) = mpsc::channel();

    thread::scope(|s| {
        for (id, chunk) in data.chunks(250).enumerate() {
            let tx = tx.clone(); // one Sender per worker
            s.spawn(move || {
                let partial: f64 = chunk.iter().sum();
                tx.send((id, partial)).unwrap(); // the tuple is moved into the channel
            });
        }
        drop(tx); // no Sender left once workers are done: the loop below ends

        let mut total = 0.0;
        for (id, partial) in &rx {
            println!("worker {id} sent {partial}");
            total += partial;
        }
        println!("total = {total}");
        assert_eq!(total, 500_500.0);
    });
}

#[test]
fn test() {
    main()
}
