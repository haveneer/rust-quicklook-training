#[test]
fn string_vs_str() {
    // String : possédée, agrandissable, tampon UTF-8 sur le tas (comme un Vec<u8> avec des garanties en plus)
    let mut owned: String = String::from("Bonjour");
    owned.push_str(", Rust");

    // &str : vue empruntée sur des octets UTF-8 (comme &[T] pour Vec<T>)
    let literal: &str = "Bonjour, Rust"; // vit dans le binaire : &'static str
    let borrowed: &str = &owned; // coercion String -> &str, comme Vec<T> -> &[T]

    assert_eq!(borrowed, literal);

    // Une fonction qui prend &str accepte aussi bien un &String (via coercion) qu'un littéral
    fn shout(s: &str) -> String {
        format!("{}!", s.to_uppercase())
    }
    assert_eq!(shout(&owned), "BONJOUR, RUST!");
    assert_eq!(shout("hi"), "HI!");
}

#[test]
fn utf8_bytes_vs_chars() {
    let s = String::from("café"); // 'é' occupe 2 octets en UTF-8

    // len() compte des OCTETS, pas des caractères
    assert_eq!(s.len(), 5); // c-a-f-é(2 octets) = 5 octets
    assert_eq!(s.chars().count(), 4); // mais bien 4 caractères

    // char : un scalaire Unicode, toujours 4 octets en mémoire (pas 1 !)
    assert_eq!(std::mem::size_of::<char>(), 4);

    // Découper par index d'octet peut couper un caractère multi-octets -> panique
    // let _ = &s[0..4]; // panic: byte index 4 is not a char boundary
    assert_eq!(s.get(0..4), None); // `get` : pas de panique, juste None si la borne est invalide
    assert_eq!(&s[0..3], "caf"); // borne valide : ok
}
