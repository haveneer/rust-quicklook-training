use std::fmt::{Display, Formatter};

pub struct ReportCard<T: Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T> ReportCard<T>
    where T: Display,
{
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

impl<T> Display for ReportCard<T>
    where T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())?;
        Ok(())
    }
}

fn main() {
    let report_card = ReportCard {
        grade: 3.14,
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    assert_eq!(
        format!("{}", report_card),
        "Gary Plotter (11) - achieved a grade of 3.14"
    );
}

#[test]
fn generate_numeric_report_card() {
    main()
}