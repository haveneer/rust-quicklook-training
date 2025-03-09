use std::ops::Deref;

// Un pointeur intelligent simple qui possède une valeur de type T
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0 // retourne une référence vers la valeur interne
    }
}

// Optionnellement, on peut implémenter DerefMut pour les références mutables:
impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn main() {
    let my = MyBox(String::from("Rust"));
    // my est de type MyBox<String>

    // Grâce à Deref, on peut appeler directement les méthodes de String via MyBox<String>:
    println!("Minuscule : {}", my.to_lowercase());
    // Explication: my.deref() retourne &String, puis deref de String retourne &str,
    // la méthode to_lowercase() (définie sur str) est donc accessible.

    // De même, on peut passer MyBox<String> à une fonction qui attend &str :
    fn greet(name: &str) {
        println!("Salut {}!", name);
    }
    greet(&my);
    // Ici, &my est de type &MyBox<String>. Le compilateur applique deref deux fois :
    // &MyBox<String> -> &String -> &str, pour correspondre à greet().
    let string_ref: &String = <MyBox<_> as Deref>::deref(&my);
    let str_ref: &str = <String as Deref>::deref(&string_ref);
    greet(str_ref);
}

#[test]
fn test() {
    main();
}
