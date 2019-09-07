//! # Domain Patterns
//!
//! This project provides patterns from the world of Domain Driven Design.
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
//!
//! # ValueObject Trait
//!
//! The `ValueObject` trait defines characteristics of a value object, which is an object that holds some immutable value, and validates
//! incoming data to make sure it conforms to certain requirements.  An example would be if you have an `Email` struct.  At all times that
//! struct should only hold valid email addresses.  If `Email` implements `ValueObject` trait, then the implementor will be required to
//! write a `try_from` implementation, which should in turn call their implementation of `validate` and essentially return an error
//! if validation fails, or create a value object upon success.  Some rules for value objects are:
//!
//! 1. Value objects are immutable.
//! 2. Value objects should validate data that is used to construct them (the "value" they hold after successful validation).
//! 3. Value objects do not have globally unique identity.

/// Models is a module that holds traits which define behaviors of various facets of a domain model, such as
/// traits that define characteristics of entities and value objects.
pub mod models;

/// Collections holds traits that define collection like abstractions. Currently it contains collection like abstractions over
/// database accesss in the form of the `Repository` pattern.
pub mod collections;

/// Event module holds the event trait that defines characteristics of all domain events.
pub mod event;

/// Command module holds traits relevant to marking commands, as well as command handler traits.
/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
pub mod command;

/// Message module holds a single marker trait that is shared by both commands and events, so command handlers can handle both commands and events.
pub mod message;
