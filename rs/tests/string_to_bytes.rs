#[rustfmt::skip]
fn main() {
    let mut s = String::from("Éléphant 🦣 épatant");
    println!("{}", s);

    let first_letter = s.get(0..1); // s[0..1] => panic! (later)
    // let first_letter = &s[0..1]; // TODO what happens if ?
    println!("First X is: {:?}", first_letter); // TODO What is the type of X ?

    println!("initial: {}/{}     {:p}", s.len(), s.capacity(), s.as_ptr());
    let letters = s.chars().collect::<Vec<char>>();
    println!("letters: {}/{}", letters.len(), letters.capacity());
    s.push('!');
    println!("after push: {}/{}  {:p}", s.len(), s.capacity(), s.as_ptr());
    s.push('!');
    println!("after push2: {}/{} {:p}", s.len(), s.capacity(), s.as_ptr());
    s.shrink_to_fit();
    println!("after fit: {}/{}   {:p}", s.len(), s.capacity(), s.as_ptr());
}

#[test]
fn test() {
    main()
}
