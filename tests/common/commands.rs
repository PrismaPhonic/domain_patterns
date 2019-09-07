use domain_patterns::command::{Command, GenericHandler, CommandHandler, MassHandler};
use std::any::Any;
use std::error::Error;
use std::collections::HashMap;
use domain_patterns::collections::Repository;
use uuid::Uuid;
use crate::common::{MockUserRepository, NaiveUser};

pub struct GenericHandle<T: Command + 'static>(Box<dyn CommandHandler<T>>);

impl<T: Command + 'static> GenericHandler for GenericHandle<T> {
    fn handle(&mut self, command: Box<dyn Any>) -> Result<(), Box<dyn Error>>{
        self.0.handle(*command.downcast::<T>().unwrap())
    }

    fn handles(&self) -> String {
        self.0.handles()
    }
}

pub struct CreateUserCommand {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Command for CreateUserCommand {
    fn kind(&self) -> String {
        "CreateUserCommand".to_string()
    }
}

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
    pub fn contains_key(&self, key: String) -> bool {
        self.repo.contains_key(&key).unwrap()
    }
}

impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    fn handle(&mut self, command: CreateUserCommand) -> Result<(), Box<dyn Error>> {
        let user = NaiveUser::new(command.id, command.first_name, command.last_name, command.email)?;
        self.repo.insert(&user);

        Ok(())
    }

    fn handles(&self) -> String {
        "CreateUserCommand".to_string()
    }
}

pub struct CommandGateway {
    // Hashmap key is a string version of the Command type.
    handlers: HashMap<String, Box<dyn GenericHandler>>,
}

impl MassHandler for CommandGateway {
    fn handle<T: Command + 'static>(&mut self, command: T) -> Result<(), Box<dyn Error>> {
        self.handlers.get_mut(&command.kind()).unwrap().handle(Box::new(command));
        Ok(())
    }

    fn register<T: Command + 'static, U: CommandHandler<T> + 'static>(&mut self, handler: U) -> Result<(), Box<dyn Error>> {
        self.handlers.insert(handler.handles(), Box::new(GenericHandle(Box::new(handler))));
        Ok(())
    }
}

impl CommandGateway {
    pub fn new() -> CommandGateway {
        CommandGateway {
            handlers: HashMap::new()
        }
    }

    // This normally wouldn't be here at all, but this is so we can get back a result in mock testing
    // The key in this case is the name of the command.
    pub fn contains_key(&self, key: String) -> bool {
        self.handlers.contains_key(&key)
    }
}

impl From<Vec<Box<dyn GenericHandler>>> for CommandGateway {
    fn from(handlers: Vec<Box<dyn GenericHandler>>) -> Self {
        let handlers_map: HashMap<String, Box<dyn GenericHandler>> = handlers.into_iter().map(|h| (h.handles(), h)).collect();
        CommandGateway {
            handlers: handlers_map,
        }
    }
}
