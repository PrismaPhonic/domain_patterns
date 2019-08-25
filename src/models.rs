use std::convert::TryFrom;
use std::hash::Hash;

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
/// #[derive(Clone)]
/// struct Email {
///     address: String,
/// }
///
/// impl PartialEq for Email {
///     fn eq(&self, other: &Self) -> bool {
///         self.address == other.address
///     }
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
///         if !Self::validate(&value) {
///             return Err(EmailValidationError);
///         }
///
///         Ok(Email {
///             address: value
///         })
///     }
/// }
///
/// impl ValueObject<String> for Email {
///     type Error = EmailValidationError;
///
///     fn validate(value: &String) -> bool {
///         let email_rx = Regex::new(
///             r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
///         ).unwrap();
///
///         email_rx.is_match(value)
///     }
///
///     fn value(&self) -> &String {
///         return &self.address;
///     }
/// }
///
/// let email = Email::try_from("test_email@email.com".to_string()).unwrap();
/// ```
pub trait ValueObject<T>: Clone + PartialEq + TryFrom<T> {
    /// The implementer of this trait must point this type at some sort of `Error`.  This `Error` should communicate that there was some
    /// kind of validation error that occurred when trying to create the value object.
    type Error;

    /// `validate` takes in incoming data used to construct the value object, and validates it against
    /// given constraints.  An example would be if we had an `Email` struct that implements `ValueObject`.
    /// The constraints we would check would ensure that the incoming data is a valid email address.
    ///
    /// Note: `validate` should be called by your implementation of `try_from`.
    fn validate(value: &T) -> bool;

    /// `value` return a reference to the internal value held in the value object. This should be the only
    /// way that we access the internal data.  Mutation methods should always generate a new value object.
    fn value(&self) -> &T;
}
