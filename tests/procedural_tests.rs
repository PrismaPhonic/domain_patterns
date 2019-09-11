#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate failure;

use domain_patterns::models::{Entity, ValueObject};
use domain_patterns::message::Message;
use domain_patterns::event::{DomainEvent,DomainEvents};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::convert::TryFrom;
use regex::Regex;

pub mod entity {
    #[derive(Entity)]
    pub struct NaiveUser {
        id: uuid::Uuid,
        version: u64,
        name: String,
        // this field will break compiling on purpose
        // for testing purposes.  should not be able to have
        // pub field with this macro.
        // pub bad_field: String,
    }

    impl NaiveUser {
        pub(crate) fn new() -> NaiveUser {
            NaiveUser {
                id: uuid::Uuid::new_v4(),
                version: 0,
                name: "Test".to_string(),
                // bad_field: "Test".to_string(),
            }
        }
    }
}

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    #[fail(display = "Email failed to validate.")]
    EmailValidationError,
}

impl ValueObject<String> for Email {
    type ValueError = Error;

    fn validate(value: &String) -> Result<(), Self::ValueError> {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        if !email_rx.is_match(value) {
            return Err(Error::EmailValidationError)
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, DomainEvent)]
pub struct FirstNameUpdatedEvent {
    pub id: Uuid,
    pub aggregate_id: String,
    pub first_name: String,
    pub version: u64,
    pub occurred: i64,
}

#[derive(Serialize, Deserialize, Clone, DomainEvent)]
pub struct EmailUpdatedEvent {
    pub id: Uuid,
    pub aggregate_id: String,
    pub email: String,
    pub version: u64,
    pub occurred: i64,
}

#[derive(Clone, DomainEvents)]
pub enum UserEvents {
    FirstNameUpdated(FirstNameUpdatedEvent),
    EmailUpdated(EmailUpdatedEvent),
}

//// UNCOMMENT THIS TO CHECK FOR COMPILE TIME FAILIURE.
//#[derive(DomainEvents)]
//pub struct NotEvents {}

#[test]
fn entity_macro_works() {
    let user = entity::NaiveUser::new();
    assert_eq!(user.name(), "Test")
}

#[test]
fn cannot_mutate_entity_fields_ever() {
    let mut user = entity::NaiveUser::new();
    let mut name = user.name();

//    // fails
//    let new_name = "NewName".to_string();
//    *user.name() = new_name;

//    // Uncomment for compile failure
//    user.name = "NewName".to_string();

    assert_ne!(user.name(), "NewName");
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
        aggregate_id: Uuid::new_v4().to_string(),
        first_name: "new_name".to_string(),
        version: 1,
        id: Uuid::new_v4(),
        occurred: 120984128912,
    };
    assert_eq!(&updated_event.aggregate_id, &updated_event.aggregate_id());
}

#[test]
fn domain_events_macro_works() {
    let updated_event = FirstNameUpdatedEvent {
        aggregate_id: Uuid::new_v4().to_string(),
        first_name: "new_name".to_string(),
        version: 1,
        id: Uuid::new_v4(),
        occurred: 120984128912,
    };
    let user_event = UserEvents::FirstNameUpdated(updated_event);
    let updated_event2 = EmailUpdatedEvent {
        aggregate_id: Uuid::new_v4().to_string(),
        email: "new_email".to_string(),
        version: 2,
        id: Uuid::new_v4(),
        occurred: 1209841289888,
    };
    let user_event2 = UserEvents::EmailUpdated(updated_event2);

    assert_eq!(user_event.version(), 1);
    assert_eq!(user_event.occurred(), 120984128912);
    assert_eq!(user_event2.version(), 2);
    assert_eq!(user_event2.occurred(), 1209841289888);
}
