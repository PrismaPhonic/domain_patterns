#[macro_use]
extern crate domain_derive;

use domain_patterns::models::{Entity, ValueObject};
use uuid::Uuid;
use std::convert::TryFrom;
use regex::Regex;

#[derive(Entity)]
struct NaiveUser {
    id: Uuid,
    version: u64,
}

impl NaiveUser {
    fn new() -> NaiveUser {
        NaiveUser {
            id: Uuid::new_v4(),
            version: 0,
        }
    }
}

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

impl ValueObject<String> for Email {
    fn validate(value: &String) -> bool {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        email_rx.is_match(value)
    }

    fn value(&self) -> &String {
        return &self.value
    }
}

#[test]
fn entity_macro_works() {
    let user = NaiveUser::new();
    assert_eq!(&user.id, &user.id())
}

#[test]
fn value_object_setup_macro_works() {
    let email = Email::try_from("test_email@email.com".to_string());
    assert!(email.is_ok());
    assert_eq!(email.unwrap().value, "test_email@email.com".to_string());
}
