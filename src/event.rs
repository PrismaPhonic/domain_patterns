use crate::models::AggregateRoot;
use crate::message::Message;

/// `DomainEvent` is a trait that defines an event relevant to the domain.  These are always facts about something
/// that has already occurred that has domain significance.  An event has a time at which the event occurred,
/// and an id that corresponds to which aggregate id the event corresponds to.  The implementor
/// should also be sure to pass the aggregates `version` in when they create the event, so that
/// events can be processed in the correct order.
pub trait DomainEvent: Message {
    /// occurred should return a timestamp as an i64.  You can generate this using a library like chrono
    /// for the current time.
    fn occurred(&self) -> i64;

    /// id is the event's id, which should be automatically generated when constructing the implementor
    /// of DomainEvent trait.  Returned value of the getter should be a String. All that matters is
    /// that you can turn the id into a string.  For instance Uuid's from the uuid crate implement
    /// Display, so you can call `.to_string()`.  Do NOT pass ownership of the underlying data (if your underlying id
    /// is a String, clone it)
    fn id(&self) -> String;

    /// aggregate_id should correlate to the id of the aggregate pushing the event down
    /// the event stream. Returned value of the getter should be a String created from underlying data.
    /// All that matters is that you can turn the id into a string.  For instance Uuid's from the uuid crate implement
    /// Display, so you can call `.to_string()`.  Do NOT pass ownership of the underlying data (if your underlying id
    /// is a String, clone it)
    fn aggregate_id(&self) -> String;

    /// version holds the version of the aggregate that the event corresponds to, which can be
    /// used to correctly order events for playback.
    fn version(&self) -> u64;
}
