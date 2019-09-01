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

/// DomainEvents is a thin wrapper over an enum that contains all generics that implement `DomainEvent` trait.
/// As far as I can tell there's no way to constrain that all variants of an enum enforce the same trait.
///
/// Note: This should be implemented on an enum once for every aggregate root.
pub trait DomainEvents {}

// EventApplier should be applied only to aggregate roots in systems where you want to use event sourcing.
pub trait EventApplier: AggregateRoot {
    type EventError;

    fn apply(&mut self, event: Self::Events) -> Result<(), Self::EventError>;
}