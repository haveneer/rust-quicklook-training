* New custom derive implementation
* 
* multi crates / modules
* ajouter un exemple de super:: et self:: dans full_tree (alternative aux chemins complets et pouvant retirer une ambiguïté en particulier pour self::)
* visibilité (pub, pub(crate), pub(super)) = https://doc.rust-lang.org/beta/rust-by-example/mod/visibility.html
* les préfixes de chaines de caractères et autres littéraux : https://doc.rust-lang.org/reference/tokens.html
* pointers variety
    * sequential : https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
    * concurrent :
    * https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
    * ```
    Box<T> is for single ownership.
    Rc<T> is for multiple ownership.
    Arc<T> is for multiple ownership, but threadsafe.
    Cell<T> is for “interior mutability” for Copy types; that is, when you need to mutate something behind a &T.
    
    Cell<T> and RefCell<T> are for single-threaded scenarios. Consider using RwLock<T> or Mutex<T> if you need shared mutability in a multi-threaded situation. 
    ```
    * compare the different "interior mutability" types
        * Cell
            * &self let's you set() the contents.
            * No references to the contents allowed. Only copying and taking.
            * Never Sync.
        * RefCell
            * &self let's you get &mut of the contents.
            * Never Sync.
        * RwLock
            * &self let's you get &mut of the contents.
            * Sync if the contents are Send + Sync.
        * Mutex
            * &self let's you get &mut of the contents.
            * Shared references to the interior also take the lock, so only one can exist at a time.
            * Sync if the contents are Send.

    * https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
* Phantom:
* const fn
* closures + move
* Generics
    * https://doc.rust-lang.org/rust-by-example/generics.html
    * https://without.boats/blog/shipping-const-generics/
* Fn, FnMut, FnOnce traits
* traits inheritance (super traits): https://doc.rust-lang.org/rust-by-example/trait/supertraits.html
* async book: https://rust-lang.github.io/async-book
* Écriture fonctionnelle (C++ range like)
* autres usages de ? et From::
  from : https://doc.rust-lang.org/beta/rust-by-example/error/multiple_error_types/reenter_question_mark.html

Bads :/

* interior mutability:
    * https://stackoverflow.com/questions/66566058/why-mutable-reference-on-const-is-not-an-error?noredirect=1#comment117674607_66566058
    * https://github.com/rust-lang/rust/issues/55721#issuecomment-643454340
    * let mut x = &mut 1; *x += 1; (ident with const)
* Do not add Copy trait if you want to avoid copies (if available, it will become the default behavior)

Meilleurs default que C++:

* no copy by default
* même le code de la std:: est lisible ! (pas un gros bazar comme la STL)

# Good questions

* https://stackoverflow.com/questions/44743409/why-can-i-use-ok-and-err-directly-without-the-result-prefix
* use xxx as _ : https://doc.rust-lang.org/std/keyword.use.html

# Must visit

* https://github.com/firecracker-microvm/firecracker

* https://github.com/CookieCoder15/pipes-rs
* https://github.com/rust-lang/rustlings
* https://blog.logrocket.com/parsing-in-rust-with-nom/

## Error management

* https://github.com/dtolnay/thiserror
* https://github.com/dtolnay/anyhow
* http://github.com/tailhook/quick-error
* https://github.com/rust-lang-nursery/error-chain

## GUI

* https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl
* https://github.com/hecrj/iced
* https://github.com/linebender/druid
* https://github.com/tauri-apps/tauri
