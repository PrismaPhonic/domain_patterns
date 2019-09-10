use domain_patterns::command::{Command, Commands, Handles, HandlesAll};
use domain_patterns::message::{Message, Messages};
use std::any::Any;
use std::collections::HashMap;
use domain_patterns::collections::Repository;
use uuid::Uuid;
use crate::common::{MockUserRepository, NaiveUser, Error};

pub struct CreateUserCommand {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Command for CreateUserCommand {
    fn kind(&self) -> &'static str {
        "CreateUserCommand"
    }
}

// TODO: Remove once we have a derive macro for this.
impl Message for CreateUserCommand {}

pub struct CreateUserCommandHandler {
    // This would be an abstraction over a database connection in a real example.
    repo: MockUserRepository,
}

impl CreateUserCommandHandler {
    pub fn new(repo: MockUserRepository) -> CreateUserCommandHandler {
        CreateUserCommandHandler {
            repo,
        }
    }

    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
    pub fn contains_key(&self, key: &String) -> bool {
        self.repo.contains_key(key).unwrap()
    }
}

impl Handles<CreateUserCommand> for CreateUserCommandHandler {
    // TODO: This is not accurate to what we would do in the real world.  make failure root node and use that
    type Error = Error;

    fn handle(&mut self, msg: &CreateUserCommand) -> Result<(), Self::Error> {
        let user = NaiveUser::new(msg.id.clone(), msg.first_name.clone(), msg.last_name.clone(), msg.email.clone())?;
        self.repo.insert(&user);

        Ok(())
    }

    fn handles(&self) -> &'static str {
        "CreateUserCommand"
    }
}

//pub struct CommandGateway {
//    // Hashmap key is a string version of the Command type.
//    handlers: HashMap<String, Box<dyn GenericHandler>>,
//}
//
//impl MassHandler for CommandGateway {
//    type Error = ValidationError;
//
//    fn handle<T: Command + 'static>(&mut self, command: T) -> Result<(), Self::Error> {
//        self.handlers.get_mut(&command.kind()).unwrap().handle(Box::new(command));
//        Ok(())
//    }
//
//    fn register<T: Command + 'static, U: CommandHandler<T> + 'static>(&mut self, handler: U) -> Result<(), Self::Error> {
//        self.handlers.insert(handler.handles(), Box::new(GenericHandle(Box::new(handler))));
//        Ok(())
//    }
//}
//
//impl CommandGateway {
//    pub fn new() -> CommandGateway {
//        CommandGateway {
//            handlers: HashMap::new()
//        }
//    }
//
//    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
//    // The key in this case is the name of the command.
//    pub fn contains_key(&self, key: String) -> bool {
//        self.handlers.contains_key(&key)
//    }
//}
//
//impl From<Vec<Box<dyn GenericHandler>>> for CommandGateway {
//    fn from(handlers: Vec<Box<dyn GenericHandler>>) -> Self {
//        let handlers_map: HashMap<String, Box<dyn GenericHandler>> = handlers.into_iter().map(|h| (h.handles(), h)).collect();
//        CommandGateway {
//            handlers: handlers_map,
//        }
//    }
//}
