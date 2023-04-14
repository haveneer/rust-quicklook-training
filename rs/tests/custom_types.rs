mod tests {
    use chrono::prelude::*;

    #[test]
    fn custom_types() {
        type ComposedName = (String, String); // anonymous tuple to define a new type

        struct Person(ComposedName); // named tuple

        // struct with named fields
        struct Birthday {
            person: Person,
            date: Date<Utc>,
        }

        // struct instanciation
        let _epoch = Birthday {
            person: Person(("Unix".into(), "Time".into())), // reuse type definition
            date: Utc.ymd(1970, 1, 1),
        };

        struct CardType; // unit struct

        enum PaymentMethod {
            Cash,                   // case without variation
            Cheque { number: u32 }, // inlined struct case definition
            Card(CardType, u64),    // tuple case definition
        }

        let _payment = PaymentMethod::Card(CardType, 42);
    }
}
