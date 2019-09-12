# domain_derive

This crate provides `domain_patterns` derive macros.

## Entity macro
The `Entity` derive macro can be used to automatically implement all methods of the `Entity` trait
from the `domain_patterns` crate.  This only works if certain preconditions are met:

1. You are applying this to a struct.
2. Your struct has an `id` field of type `Uuid`.
3. Your struct has a `version` field which is some integer type.

```edition2018
#[macro_use]
extern crate domain_derive;
use uuid::Uuid;

#[derive(Entity)]
struct User {
    id: Uuid,
    version: u64
};
```

## ValueSetup macro
The `ValueSetup` derive macro can be used to setup as much boilerplate as possible
for your choosen value object.  It checks some preconditions:

1. You are applying this to a struct.
2. Your struct has a single field called `value` of any type that is clonable.

Once you've used this macro, you will still need to implement the `ValueObject` trait,
but you will not have to implement `TryFrom` (or create the validation error for `TryFrom`, this
is handled by the macro), or implement `PartialEq` or `Clone`.

In case you need to use the validation error elsewhere, the created validation error will be the
name of your struct with ValidationError appended.  For example, if you have an `Email` struct,
then the generated validation error will be called `EmailValidationError`.

```edition2018
#[macro_use]
extern crate domain_derive;

use domain_patterns::ValueObject;
use regex::Regex;

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

impl ValueObject<String> for Email {
    fn validate(value: &String) -> bool {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        email_rx.is_match(value)
    }

    fn value(&self) -> &String {
        return &self.value;
    }
}
```

## DomainEvent macro
The `DomainEvent` macro should be applied to a struct that represents a DomainEvent. It completely
implements all methods of the `DomainEvent` trait, as long as some preconditions are met:

1. You are applying this to a struct.
2. There needs to be an `id` field of type `Uuid`.
3. There needs to be a version field of any integer type (floating point not allowed).
4. There needs to be an `aggregate_id` field of type `Uuid`.
5. There needs to be an `occurred` field of type `i64`.

```edition2018
#[macro_use]
extern crate domain_derive;

use uuid::Uuid;
use domain_patterns::event::DomainEvent;

#[derive(Serialize, Clone, DomainEvent)]
pub struct FirstNameUpdatedEvent {
    pub aggregate_id: Uuid,
    pub first_name: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}
```

## DomainEvents macro
The `DomainEvents` macro should be applied to an enum that holds variants which are all Domain Events.
This is a very thin wrapper, and all the macro does is check that the structure is an enum, and then applies
the trait, which has no methods.

```edition2018
#[macro_use]
extern crate domain_derive;

use uuid::Uuid;
use domain_patterns::event::{DomainEvent, DomainEvents};

#[derive(Serialize, Clone, DomainEvent)]
pub struct FirstNameUpdatedEvent {
    pub aggregate_id: Uuid,
    pub first_name: String,
    pub version: u64,
    pub id: Uuid,
    pub occurred: i64,
}

#[derive(Clone, DomainEvents)]
pub enum UserEvents {
    FirstNameUpdated(FirstNameUpdatedEvent),
}
```

License: MIT
