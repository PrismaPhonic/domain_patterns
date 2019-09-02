#[macro_use]
extern crate domain_derive;

use domain_patterns::models::Entity;
use uuid::Uuid;

#[derive(Entity)]
struct NaiveUser {
    id: Uuid,
    version: u64,
}

impl NaiveUser {
    fn new() -> NaiveUser {
        NaiveUser {
            id: Uuid::new_v4(),
            version: 0,
        }
    }
}

#[test]
fn it_works() {
    let user = NaiveUser::new();
    assert_eq!(&user.id, &user.id())
}
