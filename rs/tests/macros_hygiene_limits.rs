// ❌ Cet exemple ne compile PAS - démontre les limites de l'hygiène

#[allow(unused)]
macro_rules! use_var_bad {
    () => {
        println!("{}", x); // ❌ Erreur !
    };
}

// ✅ Solution : passer les variables en paramètre
macro_rules! use_var {
    ($var:expr) => {
        println!("{}", $var); // ✅
    };
}

fn main() {
    let x = 42;
    // use_var_bad!(); // Ne compile pas
    use_var!(x); // Fonctionne
}

#[test]
fn test() {
    main()
}
