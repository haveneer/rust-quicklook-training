#![feature(trace_macros)]
#![feature(log_syntax)] // requires nightly build

// To expand macro by compiler
// cargo +nightly rustc --test macro_debugging -- -Z unstable-options --pretty=expanded
// cargo +nightly rustc --test macro_debugging -- -Z macro-backtrace
// cargo expand --test macro_debugging

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*) => {each_tt!($($rest)*);}
}

#[test]
fn test_trace_macro() {
    trace_macros!(true);
    each_tt!(spim wak plee whum);
    trace_macros!(false);
    each_tt!(spim wak plee whum);
}

macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) =>
        {{ // without double brace, it does not compile (or use sing!{ })
            log_syntax!($tt); sing!($($rest)*);  // with log_syntax, compiler will print all seen tokens
        }};                                      // (compile time, not runtime)
}

#[test]
fn test_log_syntax() {
    sing!("Hello" 1 2 * 66)
}
