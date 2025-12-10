// Le trait doit hériter de 'Any' pour avoir accès à la méthode type_id()
// Sans `: Any`, item.type_id() retournerait toujours TypeId::of::<dyn Composant>()
// (le type du trait object), et non le type concret (Bouton ou Slider).
// La contrainte `: Any` permet au dispatch dynamique de retourner le vrai type sous-jacent.
trait Composant: std::any::Any {
    fn draw(&self);
}

// Quelques implémentations de Composant
struct Bouton {
    label: &'static str,
}

struct Slider {
    valeur: i32,
}

impl Composant for Bouton {
    fn draw(&self) {
        println!("Dessin d'un bouton: {}", self.label);
    }
}

impl Composant for Slider {
    fn draw(&self) {
        println!("Dessin d'un slider: {}%", self.valeur);
    }
}

fn main() {
    // Une liste hétérogène de composants (dyn Composant)
    let elements: Vec<Box<dyn Composant>> = vec![
        Box::new(Bouton { label: "Envoyer" }),
        Box::new(Slider { valeur: 50 }),
        Box::new(Bouton { label: "Annuler" }),
    ];

    // On récupère l'ID du type concret "Bouton" pour comparer
    let id_bouton = std::any::TypeId::of::<Bouton>();
    let id_slider = std::any::TypeId::of::<Slider>();

    println!("--- Analyse des types ---");

    for item in &elements {
        // L'appel à as_ref() déréférence la Box pour accéder au trait object
        // .type_id() est disponible car Composant hérite de Any
        let id_actuel = item.as_ref().type_id(); //

        if id_actuel == id_bouton {
            println!("C'est un Bouton !");
        } else if id_actuel == id_slider {
            println!("C'est un Slider !");
        } else {
            println!("Type inconnu");
        }

        // On peut toujours appeler les méthodes du trait
        item.draw();
    }
}

#[test]
fn test() {
    main()
}
