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
//! Unlike the standard libraries `HashMap` api, the `insert` does not update the value at the key, if the key already exists.
//! This is to prevent misuse of the repository.  The logic is flipped from `HashMap`'s `insert` method.  If the key already
//! exists, then `None` is returned.  If the key does not exist, then the entity itself is returned.  This is useful for cases
//! in which we want to update an entity with computed data from a database and return that to the caller.
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

use std::hash::Hash;

/// A trait that provides a collection like abstraction over database access.
///
/// Generic `T` is some struct that implements `Entity<K>` where `K` is used as the key in the repository methods.  In other words
/// it's expected that an entities id is used as the key for insert and retrieval.
pub trait Repository<K: Hash + Eq, T: Entity<K>> {
    /// The implementer of this trait must point this type at some sort of `Error`.  This `Error` should communicate that there was some
    /// kind of problem related to communication with the underlying database.
    type Error;

    /// Inserts an entity into the underlying persistent storage (MySQL, Postgres, Mongo etc.).
    ///
    /// Entity should be inserted at it's globally unique id. It implements the [`Entity`] interface,
    /// so it's globally unique id can be accessed by calling [`id()`].
    ///
    /// If the underlying storage did not have this key present, then insert is successful and the entity is returned.
    /// It might be returned with updated (computed) data that was computed by the database.
    ///
    /// If the underlying storage does have the key present, then [`None`] is returned.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Entity`]: ./trait.Entity.html
    /// [`id()`]: ./trait.Entity.html#tymethod.id
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn insert(&mut self, entity: &T) -> Result<Option<T>, Self::Error>;

    /// Returns the entity corresponding to the supplied key as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn get(&self, key: &K) -> Result<Option<T>, Self::Error>;


    /// Returns a `Vec<T>` of entities, based on the supplied `page_num` and `page_size`.
    /// The page_num should start at 1, but is up to the implementer to design as they see fit.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<T>, Self::Error>;

    /// Returns `true` if the underlying storage contains an entity at the specified key,
    /// and otherwise returns `false`.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn contains_key(&self, key: &K) -> Result<bool, Self::Error>;

    /// Updates the entity in the underlying storage mechanism and returns the up to date
    /// entity to the caller.  If the entity does not exist in the database (it's unique
    /// id is not in use), then we return [`None`].
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn update(&mut self, entity: &T) -> Result<Option<T>, Self::Error>;

    /// Removes an entity from the underlying storage at the given key,
    /// returning the entity at the key if it existed, and otherwise returning [`None`]
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn remove(&mut self, key: &K) -> Result<Option<T>, Self::Error>;
}

/// A trait that defines an `Entity`, which is any object with a unique and globally persistent identity.
///
/// The generic type `K` should match the same type as the internal globally unique id used for the entity.
/// Be careful when choosing what to return here.  The result of [`id()`] will be used as the primary key
/// for the entity when communicating with a database via a repository.
///
/// # Example
/// ```rust
/// use repository_pattern::Entity;
///
/// struct User {
///     user_id: String,
///     email: String,
///     password: String,
/// }
///
/// impl Entity<String> for User {
///     fn id(&self) -> String {
///         self.user_id.clone()
///     }
/// }
/// ```
///
/// [`id()`]: ./trait.Entity.html#tymethod.id
pub trait Entity<K: Hash + Eq> {
    fn id(&self) -> K;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::{fmt, error};

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

    #[derive(Debug, Clone)]
    struct MockDbError;

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

        fn contains_key(&self, key: &String) -> Result<bool, Self::Error> {
            let result = self.data.contains_key(key);
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
        user_repo.insert(&test_user);
        let success_result = user_repo.get(&user_id).unwrap();

        assert_eq!(&success_result.unwrap().first_name, &test_user.first_name)
    }

    #[test]
    #[allow(unused)]
    fn test_cant_add_duplicate() {
        let user_id = "test_id".to_string();
        let test_user = NaiveUser {
            user_id: user_id.clone(),
            first_name: "first_name".to_string(),
            last_name: "test_lname".to_string(),
            email: "test_email".to_string()
        };
        let mut user_repo = MockUserRepository::new();
        let returned_entity = user_repo.insert(&test_user).unwrap();
        assert!(returned_entity.is_some());

        let success_result = user_repo.get(&user_id).unwrap();
        assert_eq!(&success_result.unwrap().first_name, &test_user.first_name);

        let failure_result = user_repo.insert(&test_user).unwrap();
        assert!(failure_result.is_none());
    }

    #[test]
    #[allow(unused)]
    fn test_update_user() {
        let user_id = "test_id".to_string();
        let mut test_user = NaiveUser {
            user_id: user_id.clone(),
            first_name: "first_name".to_string(),
            last_name: "test_lname".to_string(),
            email: "test_email".to_string()
        };
        let mut user_repo = MockUserRepository::new();
        let returned_entity = user_repo.insert(&test_user).unwrap();
        assert!(returned_entity.is_some());

        let updated_name = "new_name".to_string();
        test_user.first_name = updated_name.clone();
        let mut updated_user = user_repo.update(&test_user).unwrap();
        // check that we get back Some() which implies updating worked.
        assert!(returned_entity.is_some());
        // Check that our name is correct in the returned (updated) user.
        assert_eq!(&updated_user.unwrap().first_name, &updated_name);

        // sanity check with fresh get and check that name was updated;
        updated_user = user_repo.get(&user_id).unwrap();
        assert_eq!(&updated_user.unwrap().first_name, &updated_name);
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
        user_repo.insert(&test_user);

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

        user_repo.insert(&test_user1);
        assert!(user_repo.contains_key(&user_id1).unwrap());
        user_repo.insert(&test_user2);
        assert!(user_repo.contains_key(&user_id2).unwrap());

        let results = user_repo.get_paged(1, 2).unwrap();
        assert_eq!(results.len(), 2)
    }
}
