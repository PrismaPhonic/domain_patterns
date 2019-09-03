#[macro_use]
extern crate domain_derive;

use domain_patterns::models::{Entity, ValueObject};
use domain_patterns::event::{DomainEvent,DomainEvents};
use serde::{Serialize, Deserialize};
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

#[derive(Serialize, Deserialize, Clone, DomainEvent)]
pub struct FirstNameUpdatedEvent {
    pub aggregate_id: Uuid,
    pub first_name: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}

#[derive(Clone, DomainEvents)]
pub enum UserEvents {
    FirstNameUpdated(FirstNameUpdatedEvent),
}

//// UNCOMMENT THIS TO CHECK FOR COMPILE TIME FAILIURE.
//#[derive(DomainEvents)]
//pub struct NotEvents {}

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

#[test]
fn domain_event_macro_works() {
    let updated_event = FirstNameUpdatedEvent {
        aggregate_id: Uuid::new_v4(),
        first_name: "new_name".to_string(),
        version: 1,
        id: Uuid::new_v4(),
        occurred: 120984128912,
    };
    assert_eq!(&updated_event.aggregate_id, updated_event.aggregate_id());
}
