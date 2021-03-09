// More details in
// * https://doc.rust-lang.org/1.7.0/book/macros.html (old version but not so bad)
// * https://doc.rust-lang.org/book/ch19-06-macros.html (most recent but lighter on macros)
// * https://doc.rust-lang.org/reference/macros-by-example.html (more details here)
// * https://github.com/DanielKeep/tlborm/ (ok but not up-to-date)

// Reminder: list of fragment specifiers
//   ident: an identifier. Examples: x; foo.
//   path: a qualified name. Example: T::SpecialA.
//   expr: an expression. Examples: 2 + 2; if true { 1 } else { 2 }; f(42).
//   ty: a type. Examples: i32; Vec<(char, String)>; &T.
//   pat: a pattern. Examples: Some(t); (17, 'a'); _.
//   stmt: a single statement. Example: let x = 3.
//   block: a brace-delimited sequence of statements. Example: { log(error, "hi"); return 12; }.
//   item: an item. Examples: fn foo() { }; struct Bar;.
//   meta: a "meta item", as found in attributes. Example: cfg(target_os = "windows").
//   tt: a single token tree.
//   lifetime: a LIFETIME_TOKEN. Example: 'a
//   vis: a possibly empty Visibility qualifier. Examples: pub, pub(super)
//   literal: matches -?LiteralExpression
// Additional rules
//   expr and stmt variables may only be followed by one of: => , ;
//   ty and path variables may only be followed by one of: => , = | ; : > [ { as where
//   pat variables may only be followed by one of: => , = | if in
//   Other variables may be followed by any token.

// Declarative matching macros
macro_rules! debug {
    (msg:$msg:literal $($x:expr);+) => {{ // the order matters !
        println!("{:=^30}", $msg);
        $(
            println!("{:<10} = {}", std::stringify!($x),$x);
        )*
    }};
    ($($x:expr);+) => {{ // use ; separator (not ,)
        debug!(msg:"Debug Info" $($x);+);
    }};
}

#[test]
fn test_debug() {
    let var = 1;
    let something_else = "string";
    debug!(var);
    debug!(var; something_else);
    debug!(1+2; something_else);

    let str = Struct { field: 42 };
    debug!(msg:"Hello" var; str);

    println!("\nOk not moved : {}", str);
}

// Not copiable structure
#[derive(Debug)]
struct Struct {
    field: i8,
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
