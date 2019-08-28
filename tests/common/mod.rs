use std::collections::HashMap;
use domain_patterns::models::{ValueObject, AggregateRoot, Entity};
use domain_patterns::event::{DomainEvent, EventStorer, DomainEvents};
use domain_patterns::collections::Repository;
use std::{fmt, error};
use regex::Regex;
use std::convert::TryFrom;
use chrono::Utc;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use uuid::Uuid;



// This is a simple example of the struct that matches the database rows events will be stored into.
// Some data from event_data is denormalized into rows for easy querying.
pub struct EventRecord<T: DomainEvents> {
    pub id: Uuid,
    pub version: u64,
    pub event_data: T,
}

//impl<T: DomainEvents> From<EventRecord<T>> for T {
//    fn from(events: EventRecord<T>) -> Self {
//        events.event_data
//    }
//}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}

impl UserCreatedEvent {
    fn new(user: &NaiveUser) -> UserCreatedEvent {
        UserCreatedEvent {
            user_id: user.user_id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.to_string(),
            version: user.version,
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

// Only making clonable for test case because we don't have a real backing database for events,
// and need to easily clone the events so we keep them in our backing hashmap and return owned
// copies back, since we will be returning owned copies when dealing with an actual datastore.
#[derive(Serialize, Deserialize, Clone)]
pub struct FirstNameUpdatedEvent {
    pub user_id: Uuid,
    pub first_name: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}

impl DomainEvent for UserCreatedEvent {
    fn occurred(&self) -> i64 {
        self.occurred
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn aggregate_id(&self) -> &Uuid {
        &self.user_id
    }

    fn version(&self) -> u64 {
        self.version
    }
}

impl DomainEvent for FirstNameUpdatedEvent {
    fn occurred(&self) -> i64 {
        self.occurred
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn aggregate_id(&self) -> &Uuid {
        &self.user_id
    }

    fn version(&self) -> u64 {
        self.version
    }
}

impl FirstNameUpdatedEvent {
    fn new(user: NaiveUser) -> FirstNameUpdatedEvent {
        FirstNameUpdatedEvent {
            user_id: user.user_id,
            first_name: user.first_name,
            version: user.version,
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

#[derive(Clone)]
pub enum UserEvents {
    UserCreatedEvent,
    FirstNameUpdatedEvent,
}

impl DomainEvents for UserEvents {}

pub struct EventStore<T: DomainEvents>
{
    pub store: Vec<EventRecord<T>>,
}

impl<T: DomainEvents + Clone> EventStore<T> {
    // helper method for mock tests
    fn records_to_events(records: &Vec<&EventRecord<T>>) -> Vec<T> {
        records.into_iter().map(|er| { er.event_data.clone() }).collect()
    }
}

// An implementation that requires Clone, on a real project probably not necessary to have events
// be clonable
impl<T: DomainEvents + Clone> EventStorer for EventStore<T> {
    type Events = T;
    fn events_by_aggregate(&self, aggregate_id: &Uuid) -> Vec<Self::Events> {
        self.store.iter()
            .filter(|e|{
            &e.id == aggregate_id
        })
            .map(|e| {
                e.event_data.clone()
            })
            .collect()
    }

    fn events_since_version(&self, aggregate_id: &Uuid, version: u64) -> Vec<Self::Events> {
        // collecting into a vector so we can sort (can't sort iterators) by versions.
        let mut ev_records: Vec<&EventRecord<T>> = self.store.iter()
            .filter(|e|{
            &e.id == aggregate_id && e.version > version
        })
            .collect();

        ev_records.sort_by(|a, b| a.version.cmp(&b.version));

        Self::records_to_events(&ev_records)
    }

    fn num_events_since_version(&self, aggregate_id: &Uuid, version: u64, num_events: u64) -> Vec<Self::Events> {
        let mut ev_records: Vec<&EventRecord<T>> = self.store.iter().filter(|e|{
            &e.id == aggregate_id &&
                e.version > version &&
                e.version <= version + num_events
        })
            .collect();

        ev_records.sort_by(|a, b| a.version.cmp(&b.version));

        Self::records_to_events(&ev_records)
    }
}

#[derive(Clone)]
pub struct Email {
    pub address: String,
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

#[derive(Debug, Clone)]
pub struct EmailValidationError;

impl fmt::Display for EmailValidationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Email failed to validate.")
    }
}

impl error::Error for EmailValidationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl TryFrom<String> for Email {
    type Error = EmailValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(EmailValidationError)
        }

        Ok(Email {
            address: value
        })
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}

impl ValueObject<String> for Email {
    type Error = EmailValidationError;

    fn validate(value: &String) -> bool {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        email_rx.is_match(value)
    }

    fn value(&self) -> &String {
        return &self.address
    }
}

pub struct NaiveUser {
    user_id: Uuid,
    version: u64,
    first_name: String,
    last_name: String,
    email: Email,
}

impl NaiveUser {
    pub fn new(user_id: Uuid, first_name: String, last_name: String, email: String) -> Result<NaiveUser, EmailValidationError> {
        Ok(NaiveUser {
            user_id,
            version: 0,
            first_name,
            last_name,
            email: Email::try_from(email)?
        })
    }

    pub fn change_fname(&mut self, new_fname: String) {
        self.first_name = new_fname;
        self.version = self.next_version();
        let created_event = UserCreatedEvent::new(self);
        // would publish event here - maybe create a mock bus for demonstration purposes.
    }

    // following getters are to replicate pattern common in many OOP language with private setters
    // and public getters.
    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}

impl Entity for NaiveUser {
    fn id(&self) -> Uuid {
        return self.user_id.clone()
    }

    fn version(&self) -> u64 {
        return self.version
    }
}

impl AggregateRoot for NaiveUser {
    type Events = UserEvents;
}

impl Clone for NaiveUser {
    fn clone(&self) -> Self {
        NaiveUser {
            user_id: self.user_id.clone(),
            version: self.version,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
        }
    }
}

// for naive testing
pub struct MockUserRepository {
    pub data: HashMap<Uuid, NaiveUser>
}

impl MockUserRepository {
    pub fn new() -> MockUserRepository {
        MockUserRepository {
            data: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockDbError;

impl fmt::Display for MockDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong at db.")
    }
}

impl error::Error for MockDbError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl Repository<NaiveUser> for MockUserRepository {
    type Error = MockDbError;

    fn insert(&mut self, entity: &NaiveUser) -> Result<Option<NaiveUser>, Self::Error> {
        let key = entity.id();

        let result = if self.contains_key(&key).unwrap() {
            None
        } else {
            self.data.insert(entity.id(), entity.clone());
            self.get(&key).unwrap()
        };

        Ok(result)
    }

    fn get(&self, key: &Uuid) -> Result<Option<NaiveUser>, Self::Error> {
        let result = if let Some(user) = self.data.get(key) {
            Some(user.clone())
        } else {
            None
        };
        Ok(result)
    }

    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<NaiveUser>, Self::Error> {
        let entire_collection: Vec<NaiveUser> = self.data
            .iter()
            .map(|(_, u)| {
                u.clone()
            }).collect();

        let result = if (page_num - 1) * page_size > entire_collection.len() {
            Vec::new()
        } else {
            let start = (page_num - 1) * page_size;
            let end = if start + page_size > entire_collection.len() {
                entire_collection.len()
            } else {
                start + page_size
            };

            entire_collection[start..end].to_vec()
        };

        Ok(result)
    }

    fn update(&mut self, entity: &NaiveUser) -> Result<Option<NaiveUser>, Self::Error> {
        let key = entity.id();

        let result = if self.contains_key(&key).unwrap() {
            self.data.insert(entity.id(), entity.clone());
            self.get(&key).unwrap()
        } else {
            None
        };

        Ok(result)
    }

    fn remove(&mut self, key: &Uuid) -> Result<Option<NaiveUser>, Self::Error> {
        let result = self.data.remove(key);
        Ok(result)
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

