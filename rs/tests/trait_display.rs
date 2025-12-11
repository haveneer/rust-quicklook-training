#[cfg(test)]
mod tests {
    use std::fmt;

    // A small showcase of implementing Display for a custom type and
    // honoring format modifiers (alternate `#`, precision, width, align, fill).
    struct Name {
        first: &'static str,
        last: &'static str,
    }

    impl fmt::Display for Name {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut flags = Vec::new();

            // Base representation toggled by the alternate flag `#`.
            let repr = if f.alternate() {
                flags.push("alternate".into());
                format!("{}, {}", self.last, self.first)
            } else {
                format!("{} {}", self.first, self.last)
            };

            if let Some(precision) = f.precision() {
                flags.push(format!("precision={}", precision));
            }

            // Delegate precision, width, alignment, and fill to the formatter.
            f.pad(&repr)?;

            if !flags.is_empty() {
                let x = flags.join(" and ");
                f.write_str(&format!(" ({} formatting flags)", x))?;
            }

            Ok(())
        }
    }

    #[test]
    fn display_custom_type_basic() {
        let n = Name {
            first: "John",
            last: "Doe",
        };

        // default Display
        assert_eq!(format!("{}", n), "John Doe");

        // alternate form (`#`) switches representation
        assert_eq!(format!("{:#}", n), "Doe, John (alternate formatting flags)");
    }

    #[test]
    fn display_custom_type_with_precision() {
        let n = Name {
            first: "John",
            last: "Doe",
        };

        // Precision limits the total number of characters written
        assert_eq!(format!("{:.1}", n), "J (precision=1 formatting flags)");
        assert_eq!(format!("{:.2}", n), "Jo (precision=2 formatting flags)");
        assert_eq!(
            format!("{:#.2}", n),
            "Do (alternate and precision=2 formatting flags)"
        );
    }

    #[test]
    fn display_custom_type_with_width_alignment_and_fill() {
        let n = Name {
            first: "John",
            last: "Doe",
        };

        // Width + right/left/center alignment (default fill is space)
        assert_eq!(format!("{:>12}", n), "    John Doe");
        assert_eq!(format!("{:<12}", n), "John Doe    ");
        assert_eq!(format!("{:^12}", n), "  John Doe  ");

        // Custom fill character combined with alignment
        assert_eq!(format!("{:-^12}", n), "--John Doe--");

        // Combine with alternate form (#) and right alignment
        assert_eq!(
            format!("{:>#12}", n),
            "   Doe, John (alternate formatting flags)"
        );

        // Combine width, alignment and precision
        assert_eq!(
            format!("{:>10.2}", n),
            "        Jo (precision=2 formatting flags)"
        );
        assert_eq!(
            format!("{:-^12.2}", n),
            "-----Jo----- (precision=2 formatting flags)"
        );
    }
}
