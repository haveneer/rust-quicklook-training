mod tests {
    use chrono::prelude::*;

    struct Person(&'static str);

    struct CardType(&'static str);

    #[test]
    fn custom_types() {
        struct Birthday {
            person: Person,
            date: Date<Utc>,
        }

        let _epoch = Birthday { person: Person("Unix Time"), date: Utc.ymd(1970, 1, 1) };

        enum PaymentMethod {
            Cash,
            Cheque { number: u32 },
            Card(CardType),
        }

        let _payment = PaymentMethod::Card(CardType("Visa"));
    }
}