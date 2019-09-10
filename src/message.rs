/// Message is a simple marker trait shared by commands and events, to allow for each application of either.
pub trait Message {}

/// Messages is a simple marker trait for an enum of commands, or an enum of events to share for group marking.
pub trait Messages {}
