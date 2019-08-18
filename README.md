# Database Abstraction Traits

This project provides a `Repository` trait and `Entity` trait.  A repository is a collection like abstraction over database
access.  We restrict it's use to that of persisting entities, which are objects that have globally unique and persistent
identities.

## Repository Trait

This trait defines characteristics of a repository, which is a collection like abstraction over
database access.  This trait is modeled very closely to function signatures used by the standard
libraries `HashMap` because that is the closest analog.  There are some key differences though, largely
around ownership.  The standard library wants to own it's values, but in the case of a collection "like"
abstraction over database access, it doesn't make sense for a repository to own it's data.  The database owns
the data and that data is passed to the repository which constructs an entity and returns that entity to the caller.

Due to the nature of the abstraction, it makes more sense for the Repository to take in references (because that's
all it needs to persist the data to an underlying storage system) and return owned values.

Just like the standard libraries `HashMap`, the `insert` method acts more like an `upsert` (if the key already exists,
it updates the value at that key, and otherwise inserts a new key-value pair).  It is up to the caller to re-use `insert`
after they have modified an entity, if they are trying to `update` that entity in the database.

The other way in which this differs from the API for the standard libraries `HashMap` is that all methods return a `Result`.
This is due to the fact that we might have a failure to communicate with the underlying storage mechanism, or a
concurrency related error that needs to be communicated back to the caller.  The success case very closely matches what you get
from the standard library `HashMap` while the failure case communicates an issue with the underlying storage mechanism.

## Entity Trait

The entity trait simply defines that an entity must have some sort of persistent identity.  This is established with a single function
signature that ensures any `Entity` must have an `id()` method that returns a globally unique id of some kind.
