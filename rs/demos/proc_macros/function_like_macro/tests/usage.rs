use function_like_macro::make_answer;

make_answer!();

#[test]
fn test_make_answer() {
    assert_eq!(answer(), 42);
}
