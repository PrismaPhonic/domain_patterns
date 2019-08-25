use chrono::{DateTime, Utc};
use std::hash::Hash;

/// `DomainEvent` is a trait that defines an event relevant to the domain.  These are always facts about something
/// that has already occurred that has domain significance.  An event has a time at which the event occurred,
/// and an id that corresponds to which aggregate id the event corresponds to.  The implementor
/// should also be sure to pass the aggregates `version` in when they create the event, so that
/// events can be processed in the correct order.
pub trait DomainEvent<T: Hash + Eq> {
    /// occurred should return the time the event occurred on.
    fn occurred() -> chrono::DateTime<Utc>;

    /// id should be used as the aggregate id, for the aggregate pushing the event down
    /// the event stream.
    fn id() -> T;

    /// version holds the version of the aggregate that the event corresponds to, which can be
    /// used to correctly order events for playback.
    fn version() -> i32;
}