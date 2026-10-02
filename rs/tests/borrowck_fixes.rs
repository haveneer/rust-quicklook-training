// Fixed versions of tests/failures/stable/borrowck_e*.rs, one function per error code.
// The slides show each function on its own (data-slice), next to the failing code.

fn total(v: &[f64]) -> f64 {
    v.iter().sum()
}

fn fix_e0382() {
    let data = vec![1.0, 2.0, 3.0];
    let t = total(&data); // lend instead of giving away
    println!("{t} over {} values", data.len());
}

fn fix_e0499() {
    let mut v = [1, 2, 3];
    let (head, tail) = v.split_at_mut(2); // two disjoint &mut, proven by the API
    head[0] += tail[0];
}

fn fix_e0502() {
    let mut cells = vec![1.0, 2.0];
    let first = cells[0]; // f64 is Copy: keep the value, not a reference
    cells.push(3.0);
    println!("{first}");
}

fn consume(_v: Vec<i32>) {}

fn fix_e0505() {
    let v = vec![1, 2, 3];
    let r = &v;
    println!("{r:?}"); // last use of the borrow comes first
    consume(v);
}

fn fix_e0506() {
    let mut dt = 0.1;
    let r = &dt;
    println!("{r}"); // the borrow ends here...
    dt = 0.05; // ...so the assignment is fine
    println!("{dt}");
}

fn fix_e0597() {
    let best;
    {
        let name = String::from("naca0012");
        best = name; // move the owner out instead of borrowing it
    }
    println!("{best}");
}

fn prefix(text: &str) -> String {
    text.to_uppercase()[..3].to_string() // return an owned value
}

// The result lives as long as both inputs
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }
    b
}

#[test]
fn fixes_compile_and_run() {
    fix_e0382();
    fix_e0499();
    fix_e0502();
    fix_e0505();
    fix_e0506();
    fix_e0597();
    assert_eq!(prefix("naca0012"), "NAC");
    assert_eq!(longest("wing", "fuselage"), "fuselage");
}

#[test]
fn borrowck_errors() {
    let t = trybuild::TestCases::new();
    for code in [
        "e0382", "e0499", "e0502", "e0505", "e0506", "e0597", "e0515", "e0106",
    ] {
        t.compile_fail(format!("tests/failures/stable/borrowck_{code}.rs"));
    }
}
