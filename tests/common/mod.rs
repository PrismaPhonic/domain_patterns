use std::collections::HashMap;
use domain_patterns::models::Entity;
use domain_patterns::collections::Repository;
use std::{fmt, error};
use uuid::Uuid;

pub mod events;
pub use events::*;

pub mod models;
pub use models::*;


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

