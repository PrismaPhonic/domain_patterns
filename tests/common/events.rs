use domain_patterns::models::Entity;
use domain_patterns::event::{DomainEvent, DomainEvents};
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


