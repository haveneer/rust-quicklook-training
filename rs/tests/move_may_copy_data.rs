struct LargeData {
    buffer: [u8; 1024], // Big buffer with 1024 octets, non-Copy type since Copy trait is not implemented
                        // buffer: Vec<u8>, // TODO What happens if this is a Vec?
}

impl LargeData {
    fn new() -> Self {
        Self {
            buffer: [42; 1024],
            // buffer: vec![42; 1024], // TODO (cf above)
        }
    }

    fn addresses(&self) -> (String, String) {
        (format!("{:p}", self), format!("{:p}", self.buffer.as_ptr()))
    }

    fn check_addresses(&self, ref_addresses: &(String, String), context: &str) {
        let addresses = self.addresses();
        if addresses.0 == *ref_addresses.0 {
            println!("✅ {context} self has same address");
        } else {
            println!("⚠️ {context} self has different address");
        }
        if addresses.1 == *ref_addresses.1 {
            println!("✅ {context} buffer has same address");
        } else {
            println!("⚠️ {context} buffer has different address");
        }
    }
    fn check_sum(&self) -> u32 {
        self.buffer.iter().map(|&b| b as u32).sum()
    }
}

fn main() {
    let data = LargeData::new();
    let addresses_in_main = data.addresses();

    // closure avec passage par référence (par défaut)
    let closure = || {
        data.check_addresses(&addresses_in_main, "in ref closure");
    };

    closure();

    // closure forçant la capture par valeur (move)
    let move_closure = move || {
        data.check_addresses(&addresses_in_main, "in move closure");
    };

    move_closure();
}

#[test]
fn move_is_maybe_copy() {
    main()
}

#[test]
fn move_is_copy_in_thread() {
    use std::thread;

    let data = LargeData::new();
    let checksum = data.check_sum();
    let addresses_in_main = data.addresses();

    // Spawn a new thread and move `data` inside the closure
    // move keyword is required since Rust requires that the closure used by thread::spawn
    // should 'static to live independently to the main stack => copy data
    // (i.e. side effect of closure by value)
    let handle = thread::spawn(move || {
        assert_eq!(checksum, data.check_sum());
        data.check_addresses(&addresses_in_main, "in thread");
    });

    handle.join().unwrap();
}
