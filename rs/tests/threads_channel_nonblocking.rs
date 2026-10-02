use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<(usize, u64)>();
    for id in 0..3 {
        let tx = tx.clone(); // one Sender per producer, dropped when its thread ends
        thread::spawn(move || {
            for step in 0..2 {
                thread::sleep(Duration::from_millis(30 * (id as u64 + 1)));
                tx.send((id, step)).unwrap();
            }
        });
    }
    drop(tx); // the original Sender must go too, or the channel never closes

    // try_recv: never blocks, the consumer can do something else meanwhile
    match rx.try_recv() {
        Ok(msg) => println!("try_recv: {msg:?}"),
        Err(TryRecvError::Empty) => println!("try_recv: nothing yet"),
        Err(TryRecvError::Disconnected) => unreachable!("producers still running"),
    }

    // recv_timeout: blocks, but at most for the given duration
    loop {
        match rx.recv_timeout(Duration::from_millis(40)) {
            Ok((id, step)) => println!("recv_timeout: producer {id} step {step}"),
            Err(RecvTimeoutError::Timeout) => println!("nothing for 40 ms"),
            Err(RecvTimeoutError::Disconnected) => break println!("end of stream"),
        }
    }
}

#[test]
fn test() {
    main()
}
