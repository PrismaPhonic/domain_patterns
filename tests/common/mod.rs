use std::collections::HashMap;
use domain_patterns::models::{ValueObject, AggregateRoot, Entity};
use domain_patterns::event::{DomainEvent, EventRepository, DomainEvents};
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
pub struct UserEventRecord {
    pub id: Uuid,
    pub aggregate_id: Uuid,
    pub version: u64,
    pub event_data: UserEvents,
}

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
    pub fn new(user: &NaiveUser) -> UserCreatedEvent {
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
    fn new(user: &NaiveUser) -> FirstNameUpdatedEvent {
        FirstNameUpdatedEvent {
            user_id: user.user_id,
            first_name: user.first_name.clone(),
            version: user.version,
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

#[derive(Clone)]
pub enum UserEvents {
    Created(UserCreatedEvent),
    Updated(FirstNameUpdatedEvent),
}

/// Note: This seems really dumb that we have to do this, but currently it seems like due to language
/// limitations regarding runtime inspection this might be the only way.
impl From<&UserEvents> for UserEventRecord {
    fn from(value: &UserEvents) -> Self {
        use UserEvents::*;
        match value {
            Created(e) => {
                UserEventRecord {
                    id: e.id().clone(),
                    aggregate_id: e.aggregate_id().clone(),
                    version: e.version(),
                    event_data: Created(e.clone()),
                }
            },
            Updated(e) => {
                UserEventRecord {
                    id: e.id().clone(),
                    aggregate_id: e.aggregate_id().clone(),
                    version: e.version(),
                    event_data: Updated(e.clone()),
                }
            }
        }
    }
}

impl DomainEvents for UserEvents {}

/// Hashmap key in this case is aggregate id.
pub struct UserEventRepository {
    store: HashMap<Uuid, Vec<UserEventRecord>>,
}

impl UserEventRepository {
    pub fn new() -> UserEventRepository {
        let store: HashMap<Uuid, Vec<UserEventRecord>> = HashMap::new();
        UserEventRepository {
            store,
        }
    }
    // helper method for mock tests
    fn records_to_events (records: &Vec<&UserEventRecord>) -> Vec<UserEvents> {
        records.into_iter()
            .map(|er| {
                er.event_data.clone()
            })
            .collect()
    }
}

// An implementation that requires Clone, on a real project probably not necessary to have events
// be clonable
impl EventRepository for UserEventRepository {
    type Events = UserEvents;

    fn events_by_aggregate(&self, aggregate_id: &Uuid) -> Option<Vec<Self::Events>> {
        if let Some(events) = self.store.get(aggregate_id) {
            let events: Vec<Self::Events> = events.iter().map(|e| { e.event_data.clone() }).collect();
            return Some(events);
        }
        None
    }

    fn events_since_version(&self, aggregate_id: &Uuid, version: u64) -> Option<Vec<Self::Events>> {
        if let Some(records) = self.store.get(aggregate_id) {
            let mut filtered_records: Vec<&UserEventRecord> = records
                .iter()
                .filter(|e| {
                    e.version > version
                })
                .collect();

            filtered_records.sort_by(|a, b| a.version.cmp(&b.version));

            return Some(Self::records_to_events(&filtered_records));
        }

        None
    }

    fn num_events_since_version(&self, aggregate_id: &Uuid, version: u64, num_events: u64) -> Option<Vec<Self::Events>> {
        if let Some(records) = self.store.get(aggregate_id) {
            let mut filtered_records: Vec<&UserEventRecord> = records
                .iter()
                .filter(|e| {
                    e.version > version &&
                    e.version > version &&
                        e.version <= version + num_events
                })
                .collect();

            filtered_records.sort_by(|a, b| a.version.cmp(&b.version));

            return Some(Self::records_to_events(&filtered_records));
        }

        None
    }

    /// retrieves by event id.
    fn get(&self, event_id: &Uuid) -> Option<Self::Events> {
        let maybe_record = self.store.iter().find_map(|(_, records)| {
            records.iter().find(|record| { record.id == *event_id })
        });
        if let Some(record) = maybe_record {
            return Some(record.event_data.clone());
        }
        None
    }

    fn contains_aggregate(&self, aggregate_id: &Uuid) -> bool {
        self.store.contains_key(aggregate_id)
    }

    fn insert(&mut self, event: &UserEvents) -> Option<Self::Events> {
        let ev_record = UserEventRecord::from(event);
        if self.contains_event(&ev_record.id) {
            None
        } else {
            if self.contains_aggregate(&ev_record.aggregate_id) {
                self.store.entry(ev_record.aggregate_id).and_modify(|e| e.push(ev_record));
            } else {
                self.store.insert(ev_record.aggregate_id, vec!(ev_record));
            }
            Some(event.clone())
        }
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
    data: HashMap<Uuid, NaiveUser>
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

