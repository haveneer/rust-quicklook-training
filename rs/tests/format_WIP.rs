// #![feature(format_args_capture)] // RFC2795 : https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html
// the feature `format_args_capture` has been stable since 1.58.0 and no longer requires an attribute to enable

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let var = 1;
        println!("{v}", v = var);
        println!("{var}");
    }
}
