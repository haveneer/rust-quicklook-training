use crossbeam_channel::{bounded, select, tick, unbounded};
use std::{thread, time::Duration};

fn main() {
    let (jobs_tx, jobs_rx) = bounded::<u32>(4); // bounded, like sync_channel
    let (done_tx, done_rx) = unbounded::<(usize, u32)>();
    thread::scope(|s| {
        for id in 0..3 {
            // Multi-consumer: the Receiver is Clone, workers share one queue
            let (jobs_rx, done_tx) = (jobs_rx.clone(), done_tx.clone());
            s.spawn(move || {
                for n in jobs_rx {
                    thread::sleep(Duration::from_millis(5));
                    done_tx.send((id, n * n)).unwrap();
                }
            });
        }
        drop(done_tx);
        s.spawn(move || (1..=6).for_each(|n| jobs_tx.send(n).unwrap()));

        // select!: wait on several channels at once (here with a periodic tick)
        let (ticker, mut sum) = (tick(Duration::from_millis(8)), 0);
        loop {
            select! {
                recv(done_rx) -> msg => match msg {
                    Ok((id, sq)) => { println!("worker {id} -> {sq}"); sum += sq; }
                    Err(_) => break, // every worker is done
                },
                recv(ticker) -> _ => println!("tick: partial sum = {sum}"),
            }
        }
        assert_eq!(sum, 91);
    });
}

#[test]
fn test() {
    main()
}
