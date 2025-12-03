use timing_macro::timing;

#[timing]
fn expensive_computation() -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

#[test]
fn test_timing() {
    let result = expensive_computation();
    assert_eq!(result, 42);
    // La macro affichera: "expensive_computation took ~100ms"
}
