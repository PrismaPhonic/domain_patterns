use std::collections::HashMap;
use domain_patterns::models::{ValueObject, Entity};
use domain_patterns::collections::Repository;
use std::{fmt, error};
use regex::Regex;
use std::convert::TryFrom;

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
}

impl Entity<String> for NaiveUser {
    fn id(&self) -> String {
        return self.user_id.clone()
    }
}

impl Clone for NaiveUser {
    fn clone(&self) -> Self {
        NaiveUser {
            user_id: self.user_id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
        }
    }
}

// for naive testing
pub struct MockUserRepository {
    pub data: HashMap<String, NaiveUser>
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

impl Repository<String, NaiveUser> for MockUserRepository {
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

    fn get(&self, key: &String) -> Result<Option<NaiveUser>, Self::Error> {
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

    fn remove(&mut self, key: &String) -> Result<Option<NaiveUser>, Self::Error> {
        let result = self.data.remove(key);
        Ok(result)
    }
}

pub fn create_test_user(user_id: &str) -> NaiveUser {
    NaiveUser {
        user_id: user_id.to_string(),
        first_name: "first_name".to_string(),
        last_name: "test_lname".to_string(),
        email: Email::try_from("test_email@email.com".to_string()).unwrap(),
    }
}

