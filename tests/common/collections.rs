use std::collections::HashMap;
use domain_patterns::models::Entity;
use domain_patterns::collections::{Repository, EventRepository};
use std::{fmt, error};
use uuid::Uuid;
use crate::common::{NaiveUser, UserEventRecord, UserEvents};

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
                self.data.insert(entity.id().clone(), entity.clone());
            self.get(key).unwrap()
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
            self.data.insert(entity.id().clone(), entity.clone());
            self.get(key).unwrap()
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
