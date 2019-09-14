#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate snafu;

use domain_patterns::collections::*;
mod common;
use common::*;
use uuid::Uuid;
use domain_patterns::command::{Command, Handles};

#[test]
#[allow(unused)]
fn test_add_user() {
    let user_id = Uuid::new_v4();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);
    let success_result = user_repo.get(&user_id.to_string()).unwrap();

    assert_eq!(&success_result.unwrap().first_name(), &test_user.first_name())
}

#[test]
#[allow(unused)]
fn test_cant_add_duplicate() {
    let user_id = Uuid::new_v4();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    let returned_entity = user_repo.insert(&test_user).unwrap();
    assert!(returned_entity.is_some());

    let success_result = user_repo.get(&user_id.to_string()).unwrap();
    assert_eq!(&success_result.unwrap().first_name(), &test_user.first_name());

    let failure_result = user_repo.insert(&test_user).unwrap();
    assert!(failure_result.is_none());
}

#[test]
#[allow(unused)]
fn test_update_user() {
    let user_id = Uuid::new_v4();
    let mut test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    let returned_entity = user_repo.insert(&test_user).unwrap();
    assert!(returned_entity.is_some());

    let updated_name = "new_name".to_string();
    test_user.change_fname(updated_name.clone());
    let mut updated_user = user_repo.update(&test_user).unwrap();
    // check that we get back Some() which implies updating worked.
    assert!(returned_entity.is_some());
    // Check that our name is correct in the returned (updated) user.
    assert_eq!(updated_user.unwrap().first_name(), &updated_name);

    // sanity check with fresh get and check that name was updated;
    updated_user = user_repo.get(&user_id.to_string()).unwrap();
    assert_eq!(updated_user.unwrap().first_name(), &updated_name);
}

#[test]
#[allow(unused)]
fn test_remove_user() {
    let user_id = Uuid::new_v4();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);

    // we first check that user is in repo
    assert!(user_repo.contains_key(&user_id.to_string()).unwrap());

    user_repo.remove(&user_id.to_string());
    assert!(!user_repo.contains_key(&user_id.to_string()).unwrap())
}

#[test]
#[allow(unused)]
fn test_get_paged() {
    let user_id1 = Uuid::new_v4();
    let test_user1 = common::create_test_user(&user_id1);

    let user_id2 = Uuid::new_v4();
    let test_user2 = common::create_test_user(&user_id2);
    let mut user_repo = MockUserRepository::new();

    user_repo.insert(&test_user1);
    assert!(user_repo.contains_key(&user_id1.to_string()).unwrap());
    user_repo.insert(&test_user2);
    assert!(user_repo.contains_key(&user_id2.to_string()).unwrap());

    let results = user_repo.get_paged(1, 2).unwrap();
    assert_eq!(results.len(), 2)
}

#[test]
#[allow(unused)]
fn test_survey_command() {
    let user_id1 = Uuid::new_v4();
    let test_user1 = common::create_test_user(&user_id1);
    let mut user_repo = MockUserRepository::new();

    let new_id = Uuid::new_v4();

    let create_user_command = CreateUserCommand {
        id: new_id.clone(),
        first_name: "test_first".to_string(),
        last_name: "test_last".to_string(),
        email: "email@email.com".to_string()
    };

    let mut user_command_handler = UserCommandsHandler::new(user_repo);
    user_command_handler.handle(&create_user_command).unwrap();

    assert!(user_command_handler.contains_key(&new_id.to_string()))
}

// Old test - needs lots of refactoring now
//#[test]
//#[allow(unused)]
//fn test_survey_command() {
//    let user_id1 = Uuid::new_v4();
//    let test_user1 = common::create_test_user(&user_id1);
//    let mut user_repo = MockUserRepository::new();
//
//    let new_id = Uuid::new_v4();
//
//    let create_user_command = CreateUserCommand {
//        id: new_id.clone(),
//        first_name: "test_first".to_string(),
//        last_name: "test_last".to_string(),
//        email: "email@email.com".to_string()
//    };
//
//    let user_command_handler = CreateUserCommandHandler::new(user_repo);
//    let mut command_gateway = CommandGateway::new();
//
//    command_gateway.register(user_command_handler);
//    command_gateway.handle(create_user_command);
//
//    assert!(command_gateway.contains_key("CreateUserCommand".to_string()));
//}
