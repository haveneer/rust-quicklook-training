macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e).unwrap());

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag)).unwrap();
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag)).unwrap();
        write_html!($w, $($rest)*);
    }};
}

#[test]
fn test_html() {
    use std::fmt::Write; // for macro; macro expanded from call site not before
    let mut out = String::new();

    write_html!(&mut out,
    html[
        head[title["Macro demo"]]
        body[h1["Macros are the best!"]]
    ]);

    println!("{}", out);

    assert_eq!(
        out,
        "<html><head><title>Macro demo</title></head>\
         <body><h1>Macros are the best!</h1></body></html>"
    );
}
