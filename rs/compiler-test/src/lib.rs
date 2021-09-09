use compiletest_rs as compiletest;

use std::path::PathBuf;

pub fn run_mode(mode: &'static str, custom_dir: Option<&'static str>, filters: Vec<&str>) {
    let mut config = compiletest::Config::default();
    config.mode = mode.parse().expect("Invalid mode");

    let dir = custom_dir.unwrap_or(mode);
    config.src_base = PathBuf::from(format!("tests/{}", dir));
    config.filters = filters.iter().map(|s| s.to_string()).collect();
    config.verbose = true;
    config.link_deps(); // Populate config.target_rustcflags with dependencies on the path
    config.clean_rmeta(); // If your tests import the parent crate, this helps with E0464
    
    compiletest::run_tests(&config);
}