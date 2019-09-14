use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("didn't find that"))]
    NotFound,

    #[snafu(display("invalid email address"))]
    EmailError,
}
