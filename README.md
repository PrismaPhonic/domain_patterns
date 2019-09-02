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
