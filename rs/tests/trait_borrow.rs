use std::borrow::Borrow;

struct MyHashMap<K, V> {
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V> MyHashMap<K, V> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    fn get<Q>(&self, _k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        println!(
            "in get, K: {}, Q: {}",
            std::any::type_name::<K>(),
            std::any::type_name::<Q>()
        );
        None // dummy impl for demo
    }

    fn funky<Q>(&self, _k: &Q)
    where
        Q: Borrow<RawString>,
    {
        println!("in funky, Q: {}", std::any::type_name::<Q>());
    }
}

struct RawString;
// Additional Borrow impl on String
impl Borrow<RawString> for String {
    fn borrow(&self) -> &RawString {
        &RawString
    }
}

fn main() {
    let map = MyHashMap::<String, ()>::new();
    let s = String::from("String");
    map.get(&s); // both possible due to String: Borrow<str>
                 // internally <String as Borrow<str>>::borrow(&s);
    map.get("toto"); // both possible

    map.funky(&RawString);
    map.funky(&s); // <=> Borrow<RawString>::borrow(s)
    map.funky(Borrow::<RawString>::borrow(&s));
}

#[test]
fn test() {
    main()
}
