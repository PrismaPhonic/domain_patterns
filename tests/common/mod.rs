use std::collections::HashMap;
use domain_patterns::models::{ValueObject, AggregateRoot, Entity};
use domain_patterns::event::DomainEvent;
use domain_patterns::collections::Repository;
use std::{fmt, error};
use regex::Regex;
use std::convert::TryFrom;
use chrono::Utc;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use uuid::Uuid;



/// Based on examples I've found we have duplicate data.  This is presumably so `event_data` can be worked
/// on directly, and we have data that will become directly accessible as sql fields.  not sure if this makes
/// the most sense actually.
///
/// id is aggregate id.
//pub struct EventRecord {
//    pub id: String,
//    pub version: u64,
//    pub event_data: dyn DomainEvent<String>,
//}

//impl EventRecord {
//    fn new(event: Box<dyn DomainEvent>) -> EventRecord {
//        EventRecord {
//            id: event.id(),
//            version: event.version(),
//            event_data: event,
//        }
//    }
//}


#[derive(Serialize, Deserialize)]
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
    fn new(user: NaiveUser) -> UserCreatedEvent {
        UserCreatedEvent {
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email.to_string(),
            version: user.version,
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

#[derive(Serialize, Deserialize)]
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

pub enum UserEvents {
    UserCreatedEvent,
    FirstNameUpdatedEvent,
}
//
//pub struct EventStore
//{
//    pub store: Vec<EventRecord>,
//}
//
//impl EventStorer<String> for EventStore {
//    /// events_by_aggregate returns a vector of pointers to events filtered by the supplied
//    /// aggregate id.
//    fn events_by_aggregate(&self, aggregate_id: &String) -> Vec<Box<dyn DomainEvent>> {
//        self.store.into_iter().filter(|e|{
//            &e.id == aggregate_id
//        }).collect()
//    }
//
//    /// events_since_version will give the caller all the events that have occurred for the given
//    /// aggregate id since the version number supplied.
//    fn events_since_version(&self, aggregate_id: &String, version: u64) -> Vec<Box<dyn DomainEvent>> {
//        let mut events: Vec<Box<dyn DomainEvent>> = self.store.into_iter().filter(|e|{
//            &e.id == aggregate_id && e.version > version
//        })
//            .map(|e| { e.event_data })
//            .collect();
//
//        events.sort_by(|a, b| a.version().cmp(&b.version()));
//
//        events
//    }
//
//    // num_events_since_version provides a vector of events of a length equal to the supplied `num_events`
//    // integer, starting from version + 1, and going up to version + num_events in sequential order.
//    //
//    // Used for re-hydrating aggregates, where the aggregate root can ask for chunks of events that occurred
//    // after it's current version number.
//    fn num_events_since_version(&self, aggregate_id: &String, version: u64, num_events: u64) -> Vec<Box<dyn DomainEvent>> {
//        let mut events: Vec<Box<dyn DomainEvent>> = self.store.into_iter().filter(|e|{
//            &e.id == aggregate_id &&
//                e.version > version &&
//                e.version <= version + num_events
//        })
//            .map(|e| { e.event_data })
//            .collect();
//
//        events.sort_by(|a, b| a.version().cmp(&b.version()));
//
//        events
//    }
//}

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
    pub user_id: Uuid,
    pub version: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
}

impl NaiveUser {
    fn new(user_id: Uuid, first_name: String, last_name: String, email: String) -> Result<NaiveUser, EmailValidationError> {
        Ok(NaiveUser {
            user_id,
            version: 0,
            first_name,
            last_name,
            email: Email::try_from(email)?
        })
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

