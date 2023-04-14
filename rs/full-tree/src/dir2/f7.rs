// pub is required to be used outside
pub fn f7() -> bool {
    true
}

// Inlined module
#[cfg(test)]
mod tests {
    use super::*; // load all symbols from parent module

    #[test]
    fn this_test_should_be_ok() {
        assert!(f7());
    }
}
