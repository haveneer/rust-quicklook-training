/// Newtypes: a temperature can no longer be passed as a pressure.
struct Kelvin(f64);
struct Pascal(f64);
#[derive(Debug, Clone, Copy)]
struct Seconds(usize);

/// Enum rather than a `bool`: the variant says what the value is.
enum Measure {
    Temperature(Kelvin),
    Pressure(Pascal),
}

struct Probe {
    samples: Vec<Option<Measure>>, // one per second; None: missing sample
}

impl Probe {
    /// `Option<Measure>` stored → `Option<&Measure>` exposed.
    #[must_use]
    fn sample(&self, t: Seconds) -> Option<&Measure> {
        self.samples.get(t.0)?.as_ref()
    }
}

fn main() {
    let probe = Probe {
        samples: vec![
            Some(Measure::Temperature(Kelvin(293.15))),
            None,
            Some(Measure::Pressure(Pascal(101_325.0))),
        ],
    };
    // if let: test and extraction in a single pattern (no is_some + unwrap)
    for t in (0..4).map(Seconds) {
        if let Some(Measure::Temperature(Kelvin(k))) = probe.sample(t) {
            println!("{t:?}: {k} K");
        }
    }
    // let ... else: early exit, the happy path stays unindented
    let Some(Measure::Pressure(Pascal(p))) = probe.sample(Seconds(2)) else {
        panic!("a pressure is expected at t = 2 s");
    };
    println!("pressure: {p} Pa");
}

#[test]
fn test() {
    main()
}
