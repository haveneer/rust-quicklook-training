use ouroboros::self_referencing;
use std::fmt::Debug;

/// Un trait d'exemple pour l'illustration
trait Op: Debug {
    fn f(&self);
}

/// Quelques implémentations de Op
#[derive(Debug)]
struct OpA;
impl Op for OpA {
    fn f(&self) {
        println!("OpA");
    }
}

#[derive(Debug)]
struct OpB;
impl Op for OpB {
    fn f(&self) {
        println!("OpB");
    }
}

#[self_referencing]
struct Container {
    /// Possède la liste d'opérateurs (trait objects).
    all_ops: Vec<Box<dyn Op>>,

    /// Projection : liste de références vers des éléments de all_ops.
    /// On indique qu'elle "emprunte" (borrows) all_ops et qu'elle doit être covariante
    /// (ce qui nous permet d'avoir des références &dyn Op).
    #[borrows(all_ops)]
    #[covariant]
    used_ops: Vec<&'this dyn Op>,
}

impl Container {
    /// Construit une instance de `Container` via le builder d'ouroboros.
    /// - On passe un `Vec<Box<dyn Op>>` en paramètre pour `all_ops`.
    /// - On crée la liste `used_ops` en empruntant les éléments qu'on veut (initialement vide ou non).
    fn from_ops(mut all_ops: Vec<Box<dyn Op>>) -> Self {
        // On crée l'objet via ContainerBuilder:
        ContainerBuilder {
            all_ops, // donne ownership à la structure
            // build_used_ops : closure qui reçoit un & (emprunt) sur all_ops
            // et construit le champ used_ops
            used_ops_builder: |_ops_ref| {
                // Pour commencer, on ne référence rien => liste vide
                Vec::new()
            },
        }
        .build()
    }

    /// Ajoute dans used_ops une référence vers l'opérateur d'indice `index` déjà présent dans all_ops.
    /// On recrée la projection used_ops pour qu'elle inclue ce nouvel emprunt.
    fn push(&mut self, index: usize) {
        // On utilise "with_mut" : la macro d’ouroboros autorise à recréer la "projection"
        // (c.-à-d. used_ops) en partant de all_ops.
        // "with_mut" reçoit une closure qui donne accès en lecture à all_ops
        // et renvoie la nouvelle version de used_ops qu'on veut.
        // Ensuite, ça met à jour self.used_ops.

        self.with_mut(|borrowed_fiels| {
            // all_ops : &mut Vec<Box<dyn Op>>
            // used_ops: &mut Vec<&dyn Op>
            // => on peut y insérer la référence vers all_ops[index]
            let reference: &dyn Op = &*borrowed_fiels.all_ops[index];
            borrowed_fiels.used_ops.push(reference);
        });
    }

    /// Appelle la méthode `f()` de chaque Op référencé dans used_ops
    fn call_used_ops(&self) {
        // On accède en lecture seule à used_ops :
        // with(|all_ops, used_ops| { ... }) => on ne modifie rien.
        self.with(|borrowed_fields| {
            for op_ref in borrowed_fields.used_ops.iter() {
                op_ref.f();
            }
        });
    }

    /// Pour debug : imprimer la structure
    fn debug_print(&self) {
        // On accède aux deux champs via "with"
        self.with(|borrowed_fields| {
            println!("all_ops has {} items:", borrowed_fields.all_ops.len());
            for (i, b) in borrowed_fields.all_ops.iter().enumerate() {
                println!("  all_ops[{}] => {:?}", i, b);
            }
            println!("used_ops has {} items:", borrowed_fields.used_ops.len());
            // "used_ops" sont des références => on ne peut pas facilement les afficher avec Debug
            // car &dyn Op doit impl Debug => il faut l'imposer sur Op.
            // J'ai donc ajouté `Debug` sur le trait Op pour illustrer.
            for (i, r) in borrowed_fields.used_ops.iter().enumerate() {
                println!("  used_ops[{}] => {:?}", i, r);
            }
        });
    }
}

#[test]
fn test_container_with_ouroboros() {
    // On crée quelques objets "Op" dans des Box
    let ops: Vec<Box<dyn Op>> = vec![Box::new(OpA), Box::new(OpB)];

    // On construit un Container avec ces ops
    let mut container = Container::from_ops(ops);

    // On référence successivement des Op existants
    container.push(0); // => référence OpA
    container.push(1); // => référence OpB
    container.push(0); // => référence OpC

    // Debug : on vérifie la structure
    container.debug_print();

    // On invoque la méthode `f()` de chaque Op référencé
    container.call_used_ops();

    // On peut bouger 'container' => comme c'est géré par ouroboros,
    // il ne sera pas déplacé en mémoire d'une façon qui invaliderait used_ops.
    let _container2 = container;
}
