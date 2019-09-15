use std::convert::TryFrom;
use std::fmt::Display;
use crate::event::DomainEvent;

/// A trait that defines an `Entity`, which is any object with a unique and globally persistent identity.
///
/// The generic type `K` should match the same type as the internal globally unique id used for the entity.
/// Be careful when choosing what to return here.  The result of [`id()`] will be used as the primary key
/// for the entity when communicating with a database via a repository.
///
/// # Example
/// ```rust
/// use domain_patterns::models::Entity;
///
/// struct User {
///     id: uuid::Uuid,
///     email: String,
///     password: String,
/// }
///
/// impl Entity for User {
///     fn id(&self) -> String {
///         self.id.to_string()
///     }
/// }
///
/// impl std::cmp::PartialEq for User {
///     fn eq(&self, other: &Self) -> bool {
///         &self.id() == &other.id()
///     }
/// }
/// ```
///
/// [`id()`]: ./trait.Entity.html#tymethod.id
pub trait Entity: PartialEq {
    /// id should be the entities globally unique id. It doesn't matter what it is internally as
    /// long as that thing can be returned as a string (implements Display from std)
    fn id(&self) -> String;
}

pub trait AggregateRoot: Entity {
    /// This type alias should point to an enum of events that the aggregate root will create and publish.
    type Events: DomainEvent;

    /// This type alias should point to the root error type for the crate.
    type Error;

    /// version is a simple integers that is incremented for every mutation.
    /// This allows us to have something like an `EntityCreated` event where we
    /// can pass versions in, and re-order the events for playback in the correct order.
    fn version(&self) -> u64;

    /// next_version simply returns the current version incremented by 1.  This default implementation
    /// should never have to be overriden.
    fn next_version(&self) -> u64 {
        self.version() + 1
    }
}

/// Applier should be implemented by aggregate roots in systems where you want to apply messages (commands or events)
/// to mutate an aggregate.
pub trait Applier: AggregateRoot {
    /// EventError should be filled in with a custom error type that indicates something went wrong when
    /// applying the event to the aggregate.
    type EventError;

    /// Apply takes in an event enum, of the type declared during the creation of the aggregate root, and
    /// internally should match to assess the specific variant.  Application of internal mutation should
    /// then depend upon the event type, and event data.  It's useful to build out other internal methods
    /// for applying each event type that `apply` can call for cleanliness.
    fn apply(&mut self, event: Self::Events) -> Result<(), Self::EventError>;
}

// TODO: Improve error handling situation for ValueObjects.  Maybe validate should return a list of errors rather
// than a boolean?
/// A trait that defines a `ValueObject` which is an immutable holder of value, that validates that value
/// against certain conditions before storing it.
///
/// # Example
/// ```rust
/// use std::{fmt, error};
/// use std::convert::TryFrom;
/// use regex::Regex;
/// use domain_patterns::models::ValueObject;
///
///
/// #[derive(Clone, PartialEq)]
/// struct Email {
///     value: String,
/// }
///
/// #[derive(Debug, Clone)]
/// struct EmailValidationError;
///
/// impl fmt::Display for EmailValidationError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "Email failed to validate.")
///     }
/// }
///
/// impl error::Error for EmailValidationError {
///     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
///         None
///     }
/// }
///
/// impl TryFrom<String> for Email {
///     type Error = EmailValidationError;
///
///     fn try_from(value: String) -> Result<Self, Self::Error> {
///         Self::validate(&value)?;
///
///         Ok(Email {
///             value,
///         })
///     }
/// }
///
/// impl ValueObject<String> for Email {
///     type ValueError = EmailValidationError;
///
///     fn validate(value: &String) -> Result<(), EmailValidationError> {
///         let email_rx = Regex::new(
///             r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
///         ).unwrap();
///
///         if !email_rx.is_match(value) {
///             return Err(EmailValidationError);
///         }
///
///         Ok(())
///     }
///
///     fn value(&self) -> String {
///         return self.value.clone()
///     }
/// }
///
/// impl fmt::Display for Email {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "{}", self.value)
///     }
/// }
///
/// let email = Email::try_from("test_email@email.com".to_string()).unwrap();
/// ```
pub trait ValueObject<T>: Clone + PartialEq + TryFrom<T> + Display {
    /// ValueError defines an error type that communicates there was a problem with validation.
    type ValueError;

    /// `validate` takes in incoming data used to construct the value object, and validates it against
    /// given constraints.  An example would be if we had an `Email` struct that implements `ValueObject`.
    /// The constraints we would check would ensure that the incoming data is a valid email address.
    ///
    /// Note: `validate` should be called by your implementation of `try_from`.
    fn validate(value: &T) -> Result<(), Self::ValueError>;

    /// `value` return a reference to the internal value held in the value object. This should be the only
    /// way that we access the internal data.  Mutation methods should always generate a new value object.
    /// Note: It's intentional that value returns an owned type.  This is necessary for enums, where we likely
    /// want to return a String after matching (since a string is how we match to figure out the variant upon value object
    /// creation), but in that case the string is created on the match in value(), and therefore we must pass back an owned
    /// value, not a ref (the string that was freshly created would be dropped at the end of value() if we try to pass
    /// back a ref of it).
    fn value(&self) -> T;
}
