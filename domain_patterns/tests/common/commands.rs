use domain_patterns::command::{Command, Handles};
use domain_patterns::message::Message;
use std::any::Any;
use std::collections::HashMap;
use domain_patterns::collections::Repository;
use uuid::Uuid;
use crate::common::{MockUserRepository, NaiveUser, Error};
use crate::common::errors::Error::NotFound;

#[derive(Command)]
pub struct CreateUserCommand {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Command)]
pub struct ChangeEmailCommand {
    pub id: Uuid,
    pub email: String,
}

#[derive(Command)]
enum UserCommands {
    CreateUserCommand(CreateUserCommand),
    ChangeEmailCommand(ChangeEmailCommand),
}

pub struct UserCommandsHandler {
    // This would be an abstraction over a database connection in a real example.
    repo: MockUserRepository,
}

impl UserCommandsHandler {
    pub fn new(repo: MockUserRepository) -> UserCommandsHandler {
        UserCommandsHandler {
            repo,
        }
    }

    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
    pub fn contains_key(&self, key: &String) -> bool {
        self.repo.contains_key(key).unwrap()
    }
}

impl Handles<CreateUserCommand> for UserCommandsHandler {
    type Error = Error;

    fn handle(&mut self, msg: &CreateUserCommand) -> Result<(), Self::Error> {
        let user = NaiveUser::new(msg.id.clone(), msg.first_name.clone(), msg.last_name.clone(), msg.email.clone())?;
        self.repo.insert(&user);

        Ok(())
    }
}

impl Handles<ChangeEmailCommand> for UserCommandsHandler {
    type Error = Error;

    fn handle(&mut self, msg: &ChangeEmailCommand) -> Result<(), Self::Error> {
        let user = self.repo.get(&msg.id.to_string())?;
        if let Some(mut u) = user {
            return Ok(u.change_email(&msg.email)?);
        }

        Err(NotFound.into())
    }
}

impl Handles<UserCommands> for UserCommandsHandler {
    type Error = Error;

    fn handle(&mut self, msg: &UserCommands) -> Result<(), Self::Error> {
        match msg {
            UserCommands::CreateUserCommand(cmd) => self.handle(cmd),
            UserCommands::ChangeEmailCommand(cmd) => self.handle(cmd),
        }
    }
}
