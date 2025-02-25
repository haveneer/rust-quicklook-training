#![allow(dead_code)]

use details::*;
use std::convert::{From, Into, TryFrom};

/// Structure métier principale
#[derive(Debug)]
struct NaiveUser {
    email: String,
}

/// Technical structure with validation
#[derive(Debug)]
struct DBUser {
    id: Id,
    role: Role,
    email: String,
}

mod details {
    /// Gestion d'erreurs lors de la conversion TryFrom
    use thiserror::Error;
    #[derive(Debug)]
    pub struct Id(String); // String but should be more robust
    #[derive(Debug)]
    pub struct Role(String);

    #[derive(Debug, Error)]
    pub enum UserConversionError {
        #[error("Invalid email address: {0}")]
        InvalidEmail(String),
    }

    pub struct Oracle;

    impl Oracle {
        pub fn get(&self, _email: &str) -> Option<(Id, Role)> {
            // Dummy implementation
            Some((Id(String::from("id")), Role(String::from("role"))))
        }
    }
}

/// Conversion "sans échec" de `DBUser` vers `User`.
impl From<DBUser> for NaiveUser {
    fn from(user: DBUser) -> Self {
        NaiveUser { email: user.email }
    }
}

/// Conversion "avec échec possible" de `User` vers `DBUser`.
impl TryFrom<(NaiveUser, &Oracle)> for DBUser {
    type Error = UserConversionError;

    fn try_from((user, oracle): (NaiveUser, &Oracle)) -> Result<Self, Self::Error> {
        let (id, role) = oracle
            .get(&user.email)
            .ok_or_else(|| UserConversionError::InvalidEmail(user.email.clone()))?;

        Ok(DBUser {
            id,
            role,
            email: user.email,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create any naive user from free parameter
    let user = NaiveUser {
        email: "john.doe@example.com".to_string(),
    };

    // Try conversion to a verified DBUser
    let db_user = DBUser::try_from((user, &Oracle))?;
    println!("NaiveUser -> DBUser conversion (faillible): {db_user:?}");

    let user: NaiveUser = db_user.into();
    // let user = NaiveUser::from(db_user); // equivalent
    println!("DBUser -> NaiveUser conversion (infaillible): {user:?}");

    Ok(())
}

#[test]
fn test() {
    main().unwrap();
}

#[test]
fn trait_std_from_into_failures() {
    let t = trybuild::TestCases::new();

    let version_path = "stable";

    t.compile_fail(format!(
        "tests/failures/{version_path}/trait_std_from_orphan.rs"
    ));
}
