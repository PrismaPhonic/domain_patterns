use std::error::Error;
use std::hash::Hash;

//! # Database Abstraction Traits
//!
//! This project provides a `Repository` trait and `Entity` trait.  A repository is a collection like abstraction over database
//! access.  We restrict it's use to that of persisting entities, which are objects that have globally unique and persistent
//! identities.
//!
//! # Repository Trait
//!
//! This trait defines characteristics of a repository, which is a collection like abstraction over
//! database access.  This trait is modeled very closely to function signatures used by the standard
//! libraries `HashMap` because that is the closest analog.  There are some key differences though, largely
//! around ownership.  The standard library wants to own it's values, but in the case of a collection "like"
//! abstraction over database access, it doesn't make sense for a repository to own it's data.  The database owns
//! the data and that data is passed to the repository which constructs an entity and returns that entity to the caller.
//!
//! Due to the nature of the abstraction, it makes more sense for the Repository to take in references (because that's
//! all it needs to persist the data to an underlying storage system) and return owned values.
//!
//! Just like the standard libraries `HashMap`, the `insert` method acts more like an `upsert` (if the key already exists,
//! it updates the value at that key, and otherwise inserts a new key-value pair).  It is up to the caller to re-use `insert`
//! after they have modified an entity, if they are trying to `update` that entity in the database.
//!
//! The other way in which this differs from the API for the standard libraries `HashMap` is that all methods return a `Result`.
//! This is due to the fact that we might have a failure to communicate with the underlying storage mechanism, or a
//! concurrency related error that needs to be communicated back to the caller.  The success case very closely matches what you get
//! from the standard library `HashMap` while the failure case communicates an issue with the underlying storage mechanism.
//!
//! # Entity Trait
//!
//! The entity trait simply defines that an entity must have some sort of persistent identity.  This is established with a single function
//! signature that ensures any `Entity` must have an `id()` method that returns a globally unique id of some kind.

/// A trait that provides a collection like abstraction over database access.
///
/// Generic `T` is some struct that implements `Entity<K>` where `K` is used as the key in the repository methods.  In other words
/// it's expected that an entities id is used as the key for insert and retrieval.
pub trait Repository<K: Hash + Eq, T: Entity<K>> {
    /// Inserts a key-entity pair into an underlying persistent storage (MySQL, Postgres, Mongo etc.).  Implementation is
    /// dependent on the persistence mechanism and up the implementer to design.
    ///
    /// If the underlying storage did not have this key present, [`None`] is returned.
    ///
    /// If the underlying storage did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn insert(&mut self, key: &K, entity: &T) -> Result<Option<T>, Box<dyn Error>>;

    /// Returns the entity corresponding to the supplied key as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn get(&self, key: &K) -> Result<Option<T>, Box<dyn Error>>;


    /// Returns a `Vec<T>` of entities, based on the supplied `page_num` and `page_size`.
    /// The page_num should start at 1, but is up to the implementer to design as they see fit.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<T>, Box<dyn Error>>;

    /// Returns `true` if the underlying storage contains an entity at the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn contains_key(&self, key: &K) -> Result<bool, Box<dyn Error>>;

    /// Removes an entity from the underlying storage at the given key,
    /// returning the entity at the key if it existed, and otherwise returning [`None`]
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn remove(&mut self, key: &K) -> Result<Option<T>, Box<dyn Error>>;
}

/// A trait that defines an `Entity`, which is any object with a unique and globally persistent identity.
pub trait Entity<Q: Hash + Eq> {
    fn id(&self) -> Q;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct NaiveUser {
        user_id: String,
        first_name: String,
        last_name: String,
        email: String,
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
    struct MockUserRepository {
        data: HashMap<String, NaiveUser>
    }

    impl MockUserRepository {
        fn new() -> MockUserRepository {
            MockUserRepository {
                data: HashMap::new(),
            }
        }
    }

    impl Repository<String, NaiveUser> for MockUserRepository {
        fn insert(&mut self, key: &String, entity: &NaiveUser) -> Result<Option<NaiveUser>, Box<dyn Error>> {
            let result = self.data.insert(key.clone(), entity.clone());
            Ok(result)
        }

        fn get(&self, key: &String) -> Result<Option<NaiveUser>, Box<dyn Error>> {
            let result = if let Some(user) = self.data.get(key) {
                Some(user.clone())
            } else {
                None
            };
            Ok(result)
        }

        fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<NaiveUser>, Box<dyn Error>> {
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

        fn contains_key(&self, key: &String) -> Result<bool, Box<dyn Error>> {
            let result = self.data.contains_key(key);
            Ok(result)
        }

        fn remove(&mut self, key: &String) -> Result<Option<NaiveUser>, Box<dyn Error>> {
            let result = self.data.remove(key);
            Ok(result)
        }
    }

    #[test]
    #[allow(unused)]
    fn test_add_user() {
        let user_id = "test_id".to_string();
        let test_user = NaiveUser {
            user_id: user_id.clone(),
            first_name: "first_name".to_string(),
            last_name: "test_lname".to_string(),
            email: "test_email".to_string()
        };
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&user_id, &test_user);
        let success_result = user_repo.get(&user_id).unwrap();

        assert_eq!(&success_result.unwrap().first_name, &test_user.first_name)
    }

    #[test]
    #[allow(unused)]
    fn test_remove_user() {
        let user_id = "test_id".to_string();
        let test_user = NaiveUser {
            user_id: user_id.clone(),
            first_name: "first_name".to_string(),
            last_name: "test_lname".to_string(),
            email: "test_email".to_string()
        };
        let mut user_repo = MockUserRepository::new();
        user_repo.insert(&user_id, &test_user);

        // we first check that user is in repo
        assert!(user_repo.contains_key(&user_id).unwrap());

        user_repo.remove(&user_id);
        assert!(!user_repo.contains_key(&user_id).unwrap())
    }

    #[test]
    #[allow(unused)]
    fn test_get_paged() {
        let user_id1 = "test_id1".to_string();
        let test_user1 = NaiveUser {
            user_id: user_id1.clone(),
            first_name: "first_name".to_string(),
            last_name: "test_lname".to_string(),
            email: "test_email".to_string(),
        };

        let user_id2 = "test_id2".to_string();
        let test_user2 = NaiveUser {
            user_id: user_id2.clone(),
            first_name: "first_name2".to_string(),
            last_name: "test_lname2".to_string(),
            email: "test_email2".to_string(),
        };
        let mut user_repo = MockUserRepository::new();

        user_repo.insert(&user_id1, &test_user1);
        assert!(user_repo.contains_key(&user_id1).unwrap());
        user_repo.insert(&user_id2, &test_user2);
        assert!(user_repo.contains_key(&user_id2).unwrap());

        let results = user_repo.get_paged(1, 2).unwrap();
        assert_eq!(results.len(), 2)
    }
}
