use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // tx moved here: no drop(tx) needed on the receiving side
        for step in 0..3 {
            thread::sleep(Duration::from_millis(30));
            tx.send(step).unwrap();
        }
    }); // thread ends, last Sender gone: end of stream

    // try_recv: never blocks, the consumer can do something else meanwhile
    match rx.try_recv() {
        Ok(step) => println!("try_recv: {step}"),
        Err(TryRecvError::Empty) => println!("try_recv: nothing yet"),
        Err(TryRecvError::Disconnected) => unreachable!("producer still running"),
    }

    // recv_timeout: blocks, but at most for the given duration
    loop {
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(step) => println!("recv_timeout: step {step}"),
            Err(RecvTimeoutError::Timeout) => println!("nothing for 20 ms"),
            Err(RecvTimeoutError::Disconnected) => break println!("end of stream"),
        }
    }

    // No Receiver left: send fails and gives the value back
    let (tx, rx) = mpsc::channel();
    drop(rx);
    assert_eq!(tx.send(42), Err(mpsc::SendError(42)));
}

#[test]
fn test() {
    main()
}
