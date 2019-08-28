use serde::Serialize;
use crate::models::AggregateRoot;
use uuid::Uuid;

/// `DomainEvent` is a trait that defines an event relevant to the domain.  These are always facts about something
/// that has already occurred that has domain significance.  An event has a time at which the event occurred,
/// and an id that corresponds to which aggregate id the event corresponds to.  The implementor
/// should also be sure to pass the aggregates `version` in when they create the event, so that
/// events can be processed in the correct order.
pub trait DomainEvent: Serialize {
    /// occurred should return a timestamp as an i64.  You can generate this using a library like chrono
    /// for the current time.
    fn occurred(&self) -> i64;

    /// id is the event's id, which should be automatically generated when constructing the implementor
    /// of DomainEvent trait.  Returned value should be a clone (do not pass ownership)
    fn id(&self) -> &Uuid;

    /// aggregate_id should correlate to the id of the aggregate pushing the event down
    /// the event stream. Returned value should be a clone (do not pass ownership)
    fn aggregate_id(&self) -> &Uuid;

    /// version holds the version of the aggregate that the event corresponds to, which can be
    /// used to correctly order events for playback.
    fn version(&self) -> u64;
}

// TODO: Change from Entity to Aggregate after creating Aggregate trait.
// Note #1: This is highly subject to change.  Not suggested for public consumption yet.
// Note #2: Treat this like a repository interface for events per resource.  For instance, an
// implementor might be a UserEventStore that stores all user related events, necessary for re-constructing
// a user aggregate from an event stream.
//
// Note #3: The implementor should hold a function for type casting event data to correct event type.
pub trait EventStorer<T: AggregateRoot>
{
    /// events_by_aggregate returns a vector of pointers to events filtered by the supplied
    /// aggregate id.
    fn events_by_aggregate(&self, aggregate_id: &Uuid) -> Vec<T::Events>;

    /// events_since_version will give the caller all the events that have occurred for the given
    /// aggregate id since the version number supplied.
    fn events_since_version(&self, aggregate_id: &Uuid, version: u64) -> Vec<T::Events>;

    // num_events_since_version provides a vector of events of a length equal to the supplied `num_events`
    // integer, starting from version + 1, and going up to version + num_events in sequential order.
    //
    // Used for re-hydrating aggregates, where the aggregate root can ask for chunks of events that occurred
    // after it's current version number.
    fn num_events_since_version(&self, aggregate_id: &Uuid, version: u64, num_events: u64) -> Vec<T::Events>;
}

//// EventApplier should be applied only to aggregate roots in systems where you want to use event sourcing.
//pub trait EventApplier<T: DomainEvent>: AggregateRoot {
//    type Error;
//
//    fn apply(event: T) -> Result<(), Self::Error>;
//}