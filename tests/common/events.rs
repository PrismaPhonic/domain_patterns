use std::collections::HashMap;
use domain_patterns::models::Entity;
use domain_patterns::event::{DomainEvent, DomainEvents};
use domain_patterns::collections::EventRepository;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::common::NaiveUser;

// This is a simple example of the struct that matches the database rows events will be stored into.
// Some data from event_data is denormalized into rows for easy querying.
pub struct UserEventRecord {
    pub id: Uuid,
    pub aggregate_id: Uuid,
    pub version: u64,
    pub event_data: UserEvents,
}

#[derive(Serialize, Deserialize, Clone, DomainEvent)]
pub struct UserCreatedEvent {
    pub aggregate_id: Uuid,
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
            aggregate_id: user.id(),
            first_name: user.first_name().clone(),
            last_name: user.last_name().clone(),
            email: user.email().to_string(),
            version: user.version(),
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

// Only making clonable for test case because we don't have a real backing database for events,
// and need to easily clone the events so we keep them in our backing hashmap and return owned
// copies back, since we will be returning owned copies when dealing with an actual datastore.
#[derive(Serialize, Deserialize, Clone, DomainEvent)]
pub struct FirstNameUpdatedEvent {
    pub aggregate_id: Uuid,
    pub first_name: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}

impl FirstNameUpdatedEvent {
    fn new(user: &NaiveUser) -> FirstNameUpdatedEvent {
        FirstNameUpdatedEvent {
            aggregate_id: user.id(),
            first_name: user.first_name().clone(),
            version: user.version(),
            id: Uuid::new_v4(),
            occurred: Utc::now().timestamp(),
        }
    }
}

#[derive(Clone, DomainEvents)]
pub enum UserEvents {
    UserCreated(UserCreatedEvent),
    FirstNameUpdated(FirstNameUpdatedEvent),
}

/// Note: This seems really dumb that we have to do this, but currently it seems like due to language
/// limitations regarding runtime inspection this might be the only way.
impl From<&UserEvents> for UserEventRecord {
    fn from(value: &UserEvents) -> Self {
        use UserEvents::*;
        match value {
            UserCreated(e) => {
                UserEventRecord {
                    id: e.id().clone(),
                    aggregate_id: e.aggregate_id().clone(),
                    version: e.version(),
                    event_data: UserCreated(e.clone()),
                }
            },
            FirstNameUpdated(e) => {
                UserEventRecord {
                    id: e.id().clone(),
                    aggregate_id: e.aggregate_id().clone(),
                    version: e.version(),
                    event_data: FirstNameUpdated(e.clone()),
                }
            }
        }
    }
}

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
