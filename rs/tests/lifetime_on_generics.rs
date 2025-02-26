#[derive(Debug)]
struct SubData<'x> {
    text: &'x str, // Une structure simple contenant une référence.
}

// Notre structure de données principale.
// - contient `data_a` qui est une référence de durée de vie `'a`.
// - contient `item` de type générique `T`. S'il contient des références,
//   alors `T` doit vivre au moins aussi longtemps que `'a` => `T: 'a`.
struct ComplexHolder<'a, T: 'a> {
    // TODO T:'a is required and cannot be inferred
    data_a: &'a SubData<'a>,
    item: T,
}

impl<'a, T: std::fmt::Debug + 'a> ComplexHolder<'a, T> {
    // Création du ComplexHolder en liant explicitement T à 'a.
    fn new(data_a: &'a SubData<'a>, item: T) -> Self {
        Self { data_a, item }
    }

    fn show(&self) {
        println!(
            "ComplexHolder -> data_a: {:?}, item: {:?}",
            self.data_a.text, self.item
        );
    }
}

// Pour illustrer le problème, on définit une fonction qui
// prend deux SubData avec des durées de vie différentes `'a` et `'b`,
// et essaye de créer un ComplexHolder<'a, &'b SubData<'b>>.
// La contrainte `T: 'a` imposera que `'b >= 'a`, ce qui peut être faux.
fn create_holder<'a, 'b>(
    data_a: &'a SubData<'a>,
    data_b: &'b SubData<'b>,
) -> ComplexHolder<'a, &'b SubData<'b>> {
    // Si on enlève la contrainte T: 'a dans ComplexHolder, on pourrait
    // tenter de compiler ce code même quand `'b < 'a`, menant à une référence invalide.
    ComplexHolder::new(data_a, data_b)
}

fn main() {
    let text_a = String::from("Hello from A");
    let text_b = String::from("Hello from B");

    let sub_a = SubData { text: &text_a }; // Vit tout le long du main
    let holder: ComplexHolder<'_, &SubData<'_>>; // pour la démo

    {
        let sub_b = SubData { text: &text_b }; // Vit seulement dans ce bloc

        // On tente de créer un holder dont la durée de vie globale est celle de 'sub_a' (liée à `'main`).
        // Mais on y insère un `sub_b` qui vit moins longtemps !
        holder = create_holder(&sub_a, &sub_b);

        // Dans l'immédiat, ça fonctionne parce que `sub_b` est encore vivant.
        holder.show();
    }
    // Ici, `sub_b` est détruit, mais `holder` reste en vie jusqu'à la fin de `main` !
    // Sans `T: 'a`, on pourrait aboutir à un code qui compile à tort et crash à l'exécution
    // (dangling reference). Grâce à `T: 'a`, le compilateur nous protège
    // et refuse ce code si `'b < 'a`.

    // NB: Selon la position des scopes, ce code est *susceptible*
    //     de déclencher une erreur de compilation car `'b < 'a`.
    // Rust vérifie que les durées de vie demandées respectent T: 'a.
    // On illustre ici pourquoi cette contrainte est nécessaire !
}

#[test]
fn test() {
    main()
}
