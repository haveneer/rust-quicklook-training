struct Recorder {
    samples: Vec<f64>,
}

impl Recorder {
    /// Hands over the samples recorded so far; the recorder starts again empty.
    fn take_samples(&mut self) -> Vec<f64> {
        // self.samples // TODO what happens if this replaces the line below?
        std::mem::take(&mut self.samples) // leaves `Vec::default()` in place
    }
}

/// A state change in place, through `&mut self`.
enum Phase {
    Warmup { samples: Vec<f64> },
    Production { warmup: Vec<f64>, samples: Vec<f64> },
}

impl Phase {
    fn start_production(&mut self) {
        if let Phase::Warmup { samples } = self {
            let warmup = std::mem::take(samples); // no clone of the samples
            *self = Phase::Production {
                warmup,
                samples: Vec::new(),
            };
        }
    }
}

fn main() {
    let mut recorder = Recorder {
        samples: vec![1.0, 2.0, 3.0],
    };
    let batch = recorder.take_samples();
    assert_eq!((batch.len(), recorder.samples.len()), (3, 0));

    let mut phase = Phase::Warmup {
        samples: vec![0.1, 0.2],
    };
    phase.start_production();
    if let Phase::Production { warmup, samples } = &phase {
        println!("warmup: {warmup:?}, production: {samples:?}");
    }
}

#[test]
fn test() {
    main()
}
