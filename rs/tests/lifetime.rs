// More info :
// * https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa
// * https://doc.rust-lang.org/1.9.0/book/lifetimes.html

#[test]
fn lifetime0() {
    let mut s = String::from("Rust");
    // Deux emprunts immuables en même temps (ok car juste en lecture)
    let r1 = &s;
    let r2 = &s;
    println!("{} et {}", r1, r2);
    // Maintenant un emprunt mutable (possible seulement après la fin des emprunts immuables)
    let r3 = &mut s;
    r3.push('y'); // modification via l'emprunt mutable
    println!("{}", r3);
}

fn max1(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

#[test]
fn lifetime1() {
    let r: i32; // copy value: no lifetime issue
    let a: i32 = { 2 + 2 };
    {
        let b: i32;
        {
            b = 5;
            r = max1(a, b);
            println!("max is {:?}", r);
        }
        println!("b is {:?}", b);
        println!("max is {:?}", r);
    }
    println!("a is {:?}", a);
    println!("max is {:?}", r);
}

fn max2<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    println!("x is {:?}", *x);
    println!("y is {:?}", *y);
    if *x > *y {
        x
    } else {
        y
    }
}

#[test]
fn lifetime2() {
    // On usage of reference:
    // https://stackoverflow.com/questions/36335342/meaning-of-the-ampersand-and-star-symbols-in-rust

    let r: &i32;
    let a: i32 = 4;
    {
        let b: i32 = 5;
        {
            r = max2(&a, &b);
            println!("max is {:?}", r);
        }
        println!("b is {:?}", b);
    }
    println!("a is {:?}", a);
    //    println!("max is {:?}", r.v); // too short lifetime if b
}

struct W {
    v: i32,
}

fn max3<'a>(x: &'a W, y: &'a W) -> &'a W {
    println!("x is {:?}", x.v);
    println!("y is {:?}", y.v);
    if x.v > y.v {
        x
    } else {
        y
    }
}

#[test]
fn lifetime_and_nested_blocks() {
    let r: &W;
    let a: W = W { v: 4 };
    {
        let b: W;
        {
            b = W { v: 5 };
            r = max3(&a, &b);
            println!("max is {:?}", r.v);
        }
        println!("b is {:?}", b.v);
        println!("max is {:?}", r.v);
    }
    println!("a is {:?}", a.v);
    //    println!("max is {:?}", r.v); // too short lifetime if b
}

#[test]
fn lifetime_with_scope_annotation() {
    // Good idea but unstable : https://github.com/rust-lang/rust/issues/48594
    //    // This is subtyping
    //    let val1 = 42;
    //    let val2 = 24;
    //    'x: {
    //        let ref1 : &'x i32 = val1;
    //        'y: {
    //            let mut ref2 : &'y mut i32= val2;
    //            ref2 = ref1;
    //        }
    //    }
}

// 'a : 'b : 'a outlives 'b <=> 'a lasts at least as long as 'b
fn f<'a, 'b>(x: &'a str, mut y: &'b str)
where
    'a: 'b,
{
    y = x; // &'a str is a subtype of &'b str because 'a: 'b
    let _r: &'b &'a i32 = &&0; // &'b &'a str is well formed because 'a: 'b
}

// See https://doc.rust-lang.org/reference/trait-bounds.html
// https://doc.rust-lang.org/reference/subtyping.html

#[test]
fn lifetime_hierarchy() {
    let x = 1.to_string();
    {
        let y = 2.to_string();
        let z = f(&x, &y);
        f(&y, &x);
        x
    };
}

struct Droppable {
    v: i32,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Droppable {{ v: {} }} has been dropped", self.v);
    }
}

#[test]
fn lifetime_explicit_drop() {
    let mut x = Droppable { v: 1 };
    x = Droppable { v: 2 }; // drop implied by assignment (cf mut)
    drop(x); // explicit drop
    let x = Droppable { v: 3 };
    // x: A binding is just a name for a value, pointing the name to something else does not affect the value itself, which lives as it would have otherwise
    let x = Droppable { v: 4 }; // new binding doesn't imply immediate drop
    println!("End of block");
}

#[test]
fn lifetime_drop_copy_is_not_drop() {
    let x = 1;
    println!("Before drop x={x}");
    drop(x); // drop a copy of y since i32 is Copy
    println!("After drop x={x} and still alive!");
}

struct Line<'a> {
    text: &'a str,
    sep: &'a str,
}

fn find_sep<'a, 'b>(text: &'a str, sep: &'b str) -> &'a str {
    let idx = text.find(sep).unwrap_or(text.len());
    &text[..idx]
}

#[test]
fn lifetime_annotations() {
    let text = "Hier, au zoo, j'ai vu dix guépards, cinq zébus, un yak et le wapiti fumer.";
    let line: Line<'_> = Line {
        text,
        sep: ", j'ai vu ",
    };
    let when_where = find_sep(line.text, line.sep);
    println!("{when_where}");
}

fn assign_refs<'a, 'b>(x: &'a i32, mut y: &'b i32) {
    // Tenter d'affecter `y = x` ici provoquerait une erreur si aucune contrainte n'est spécifiée :
    // y = x; // Erreur : `x` vit peut-être moins longtemps que `y` (pas de garantie que `'a : 'b`)
}

fn assign_refs_outlive<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
    'a: 'b,
{
    // Ici, on spécifie que 'a dure au moins aussi longtemps que 'b.
    y = x; // OK : on peut assigner une référence &'a i32 à une variable de type &'b i32 car 'a: 'b.
}

#[test]
fn lifetime_with_explicit_bounds() {
    let text = "Hier, au zoo, j'ai vu dix guépards, cinq zébus, un yak et le wapiti fumer.";
    let line: Line<'_> = Line {
        text,
        sep: ", j'ai vu ",
    };
    let when_where = find_sep(line.text, line.sep);
    println!("{when_where}");
}

#[test]
fn lifetime_with_static_bounds() {
    let literal_message: &'static str = "Hello";
    let message: String = String::from("World");
    let msg_ref: &'_ str = message.as_str(); // borrow `message`

    std::thread::spawn({
        let message = message.clone();
        move || {
            // std::thread::sleep(std::time::Duration::from_millis(100));
            println!("{},", literal_message); // 'static: available anywhere
            println!("\t{};", message); // moved in (available locally)

            // println!("{}", msg_ref); // KO: does not live long enough
            //                          // The thread may last after `main` scope
        }
    });

    std::thread::scope(|scope| {
        // Requires Rust 1.63+
        scope.spawn(|| {
            println!("{},", literal_message); // 'static: available anywhere
            println!("\t{}.", msg_ref); // OK: scoped thread bounds the lifetimes
        });
    });
}

#[test]
#[rustfmt::skip]
fn lifetime_failures() {
    let t = trybuild::TestCases::new();

    let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail(format!("tests/failures/{version_path}/missing_lifetime.rs"));
    t.compile_fail(format!("tests/failures/{version_path}/bad_lifetime.rs"));
}
