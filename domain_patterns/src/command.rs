use crate::message::Message;

/// Command is a simple marker trait for command structs.  These are commands that are issued and handled
/// by a command handler.  They are things we can say "no" to.
pub trait Command: Message {}

/// Command handler will handle any generic message.  This could be used in a 1:1 fashion, with only
/// one handler per command, or you could implement this on a single handler that handles multiple messages,
/// and lastly implement it on an enum that holds variants of those same commands.  The enum implementation
/// can simply match on self, and then call .handle which will use the incoming variant type to call the appropriate
/// generic Handles implementation.
pub trait Handles<T: Message> {
    type Result;

    fn handle(&mut self, msg: T) -> Self::Result;
}
