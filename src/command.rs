use crate::message::Message;

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Command is a simple marker trait for command structs.  These are commands that are issued and handled
/// by a command handler.  They are things we can say "no" to.
pub trait Command: Message {
    fn kind(&self) -> &'static str;
}

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Command handler will handle a single command only.
pub trait Handles<T: Message> {
    type Error;

    fn handle(&mut self, msg: &T) -> Result<(), Self::Error>;
}
