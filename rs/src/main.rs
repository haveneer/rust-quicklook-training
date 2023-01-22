fn main() {
    #[cfg(feature = "nightly")]
    let feature = "nightly";
    #[cfg(not(feature = "nightly"))]
    let feature = "!nightly";

    if cfg!(feature = "nightly") {
        println!("Running nightly/{feature}");
    } else {
        println!("Running !nightly/{feature}");
    };
}
