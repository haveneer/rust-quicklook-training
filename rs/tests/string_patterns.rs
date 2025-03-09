use arcstr::ArcStr;
use std::sync::Arc;

#[test]
fn main() {
    println!("Box<str>");
    let s = String::from("hello");
    println!("{:p}", s.as_ptr());
    let x: Box<str> = s.into();
    println!("{:p}", x.as_ptr());

    println!("Arc<str>");
    let s = String::from("hello");
    println!("{:p}", s.as_ptr());
    let x: Arc<str> = s.into();
    println!("{:p}", x.as_ptr()); // NOT SAME POINTER => NEW ALLOCATION

    // La représentation interne d’un Arc<str> inclut ces deux compteurs, généralement placés avant les octets de la chaîne dans le même bloc mémoire.
    // Le problème est que le buffer d’un String initial n’a pas prévu cet espace supplémentaire. On ne peut donc pas simplement “réutiliser” le buffer d’origine,

    println!("Rc<str>");
    let s = String::from("hello");
    println!("{:p}", s.as_ptr());
    let x: std::rc::Rc<str> = s.into();
    println!("{:p}", x.as_ptr()); // NOT SAME POINTER => NEW ALLOCATION

    println!("ArcStr"); // voir aussi arcstr::litteral! no cost for litteral
    let s = String::from("hello");
    println!("{:p}", s.as_ptr());
    let x: ArcStr = s.into();
    println!("{:p}", x.as_ptr()); // NOT SAME POINTER => NEW ALLOCATION

    println!("ArcStr::litteral!"); // no cost for litteral
    let x: ArcStr = arcstr::literal!("hello");
    println!("{:p}", x.as_ptr());
}
