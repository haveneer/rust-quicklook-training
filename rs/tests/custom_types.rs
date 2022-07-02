mod tests {
    use chrono::prelude::*;

    #[test]
    fn custom_types() {
        struct Person(&'static str);

        struct Birthday {
            person: Person,
            date: Date<Utc>,
        }

        let _epoch = Birthday {
            person: Person("Unix Time"),
            date: Utc.ymd(1970, 1, 1),
        };

        struct CardType(&'static str);

        enum PaymentMethod {
            Cash,
            Cheque { number: u32 },
            Card(CardType),
        }

        let _payment = PaymentMethod::Card(CardType("Visa"));
    }
}
