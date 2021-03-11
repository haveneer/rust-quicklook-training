* multi crates / modules
* visibilité (pub, pub(crate), pub(super)) = https://doc.rust-lang.org/beta/rust-by-example/mod/visibility.html
* les préfixes de chaines de caractères et autres littéraux : https://doc.rust-lang.org/reference/tokens.html
* pointers variety
  * sequential : https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
  * concurrent : 
  * https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
* Phantom: 
* Rust + CXX : https://github.com/dtolnay/cxx
* Rust + Python3
* const fn
* closures + move  
* Generics
  * https://doc.rust-lang.org/rust-by-example/generics.html
  * https://without.boats/blog/shipping-const-generics/
* Fn, FnMut, FnOnce traits
* traits inheritance (super traits): https://doc.rust-lang.org/rust-by-example/trait/supertraits.html
* async book: https://rust-lang.github.io/async-book
* Écriture fonctionnelle (C++ range like)


Bads :/
* interior mutability: 
    * https://stackoverflow.com/questions/66566058/why-mutable-reference-on-const-is-not-an-error?noredirect=1#comment117674607_66566058
    * https://github.com/rust-lang/rust/issues/55721#issuecomment-643454340
* Do not add Copy trait if you want to avoid copies (if available, it will become the default behavior)  


Meilleurs default que C++:
* no copy by default
* même le code de la std:: est lisible ! (pas un gros bazar comme la STL)


# Good questions
* https://stackoverflow.com/questions/44743409/why-can-i-use-ok-and-err-directly-without-the-result-prefix
* use xxx as _ : https://doc.rust-lang.org/std/keyword.use.html 