#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    #[fail(display = "Email failed to validate.")]
    EmailError,

    #[fail(display = "Something went wrong with the database.")]
    MockDbError,
}
