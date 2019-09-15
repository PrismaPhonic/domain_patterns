use domain_patterns::models::{ValueObject, AggregateRoot, Entity};
use regex::Regex;
use std::convert::TryFrom;
use uuid::Uuid;
use crate::common::{UserEvents, UserCreatedEvent, Error};
use crate::common::errors::Error::EmailError;

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

impl ValueObject<String> for Email {
    type ValueError = Error;

    fn validate(value: &String) -> Result<(), Error> {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        if !email_rx.is_match(value) {
            return Err(EmailError);
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Entity, Clone)]
pub struct NaiveUser {
    id: Uuid,
    version: u64,
    first_name: String,
    last_name: String,
    email: Email,
}

impl AggregateRoot for NaiveUser {
    type Events = UserEvents;

    type Error = Error;

    fn version(&self) -> u64 {
        self.version as u64
    }
}

impl NaiveUser {
    pub fn new(user_id: Uuid, first_name: String, last_name: String, email: String) -> Result<NaiveUser, Error> {
        Ok(NaiveUser {
            id: user_id,
            version: 0,
            first_name,
            last_name,
            email: Email::try_from(email)?
        })
    }

    pub fn change_fname(&mut self, new_fname: String) {
        self.first_name = new_fname;
        self.version = self.next_version();
        let _created_event = UserCreatedEvent::new(self);
        // would publish event here - maybe create a mock bus for demonstration purposes.
    }

    pub fn change_email(&mut self, new_email: &String) -> Result<(), Error> {
        self.email = Email::try_from(new_email.clone())?;
        self.version = self.next_version();
        let _created_event = UserCreatedEvent::new(self);
        // would publish event here - maybe create a mock bus for demonstration purposes.

        Ok(())
    }
}

pub fn create_test_user(user_id: &Uuid) -> NaiveUser {
    // TODO: Update to return a Result type and pass error back.
    NaiveUser::new(
        user_id.clone(),
        "first_name".to_string(),
        "test_lname".to_string(),
        "test_email@email.com".to_string(),
    ).unwrap()
}
