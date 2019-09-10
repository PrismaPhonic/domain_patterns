use domain_patterns::command::{Command, Handles};
use domain_patterns::message::Message;
use std::any::Any;
use std::collections::HashMap;
use domain_patterns::collections::Repository;
use uuid::Uuid;
use crate::common::{MockUserRepository, NaiveUser, Error, ErrorKind};
use crate::common::ErrorKind::NotFound;

pub struct CreateUserCommand {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub struct ChangeEmailCommand {
    pub id: Uuid,
    pub email: String,
}

impl Command for CreateUserCommand {
    fn kind(&self) -> &'static str {
        "CreateUserCommand"
    }
}

impl Command for ChangeEmailCommand {
    fn kind(&self) -> &'static str {
        "ChangeEmailCommand"
    }
}

// TODO: Remove once we have a derive macro for this.
impl Message for CreateUserCommand {}
impl Message for ChangeEmailCommand {}

enum UserCommands {
    CreateUserCommand(CreateUserCommand),
    ChangeEmailCommand(ChangeEmailCommand),
}

impl Command for UserCommands {
    fn kind(&self) -> &'static str {
        match self {
            UserCommands::CreateUserCommand(c) => c.kind(),
            UserCommands::ChangeEmailCommand(c) => c.kind(),
        }
    }
}

impl Message for UserCommands {}

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

//pub struct CreateUserCommandHandler2<'a> {
//    // This would be an abstraction over a database connection in a real example.
//    repo: &'a mut MockUserRepository,
//}
//
//impl<'a> CreateUserCommandHandler2<'a> {
//    pub fn new(repo: &'a mut MockUserRepository) -> CreateUserCommandHandler2<'a> {
//        CreateUserCommandHandler2 {
//            repo,
//        }
//    }
//
//    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
//    pub fn contains_key(&self, key: &String) -> bool {
//        self.repo.contains_key(key).unwrap()
//    }
//}
//
//impl<'a> Handles<CreateUserCommand> for CreateUserCommandHandler2<'a> {
//    type Error = Error;
//
//    fn handle(&mut self, msg: &CreateUserCommand) -> Result<(), Self::Error> {
//        let user = NaiveUser::new(msg.id.clone(), msg.first_name.clone(), msg.last_name.clone(), msg.email.clone())?;
//        self.repo.insert(&user);
//
//        Ok(())
//    }
//
//    fn handles(&self) -> &'static str {
//        "CreateUserCommand"
//    }
//}
//
//pub struct ChangeEmailCommandHandler2<'a> {
//    // This would be an abstraction over a database connection in a real example.
//    repo: &'a mut MockUserRepository,
//}
//
//impl<'a> ChangeEmailCommandHandler2<'a> {
//    pub fn new(repo: &mut MockUserRepository) -> ChangeEmailCommandHandler2<'a> {
//        ChangeEmailCommandHandler {
//            repo,
//        }
//    }
//
//    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
//    pub fn contains_key(&self, key: &String) -> bool {
//        self.repo.contains_key(key).unwrap()
//    }
//}
//
//impl<'a> Handles<ChangeEmailCommand> for ChangeEmailCommandHandler2<'a> {
//    type Error = Error;
//
//    fn handle(&mut self, msg: &ChangeEmailCommand) -> Result<(), Self::Error> {
//        let user = self.repo.get(&cmd.id.to_string())?;
//        if let Some(mut u) = user {
//            u.change_email(&cmd.email)
//        } else {
//            return Err(NotFound.into())
//        }
//
//        Ok(())
//    }
//
//    fn handles(&self) -> &'static str {
//        "ChangeEmailCommand"
//    }
//}
//
//pub enum UserHandlers<'a> {
//    CreateUserCommandHandler2(CreateUserCommandHandler2<'a>),
//    ChangeEmailCommandHandler2(ChangeEmailCommandHandler2<'a>),
//}
//
//
//// This is a rough sketch of an idea where we have a handler for all the commands related to a resource.
//pub struct UserCommandsHandler2<'a> {
//    // This would be an abstraction over a database connection in a real example.
//    repo: MockUserRepository,
//
//    handlers: HashMap<&'static str, UserHandlers<'a>>
//}
//
//impl<'a> UserCommandsHandler2<'a> {
//    pub fn new(mut repo: MockUserRepository) -> UserCommandsHandler2<'a> {
//        let create_user_handler = CreateUserCommandHandler2::new(&mut repo);
//        let change_email_handler = ChangeEmailCommandHandler2::new(&mut repo);
//        let mut handlers = HashMap::new();
//        handlers.insert(create_user_handler.handles(), UserHandlers::CreateUserCommandHandler2(create_user_handler));
//        handlers.insert(change_email_handler.handles(), UserHandlers::ChangeEmailCommandHandler2(change_email_handler));
//
//        UserCommandsHandler2 {
//            repo,
//            handlers,
//        }
//    }
//
//    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
//    pub fn contains_key(&self, key: &String) -> bool {
//        self.repo.contains_key(key).unwrap()
//    }
//
//    pub fn get_handler(&mut self, msg: &UserCommands) -> &'a mut UserHandlers {
//        self.handlers.get_mut(msg.kind()).unwrap()
//    }
//}
//
//impl<'a> HandlesAll<UserCommands> for UserCommandsHandler2<'a> {
//    type Error = Error;
//
//    fn handle(&mut self, msg: &UserCommands) -> Result<(), Self::Error> {
//        match msg {
//            UserCommands::CreateUserCommand(c) => {
//                if let UserHandlers::CreateUserCommandHandler2(h) = self.get_handler(msg) {
//                    return h.handle(c);
//                }
//            },
//            UserCommands::ChangeEmailCommand(c) => {
//                if let UserHandlers::ChangeEmailCommandHandler2(h) = self.get_handler(msg) {
//                    return h.handle(c);
//                }
//            }
//        };
//
//        Err(ErrorKind::MockDbError.into())
//    }
//}
