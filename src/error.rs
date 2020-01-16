use std::io;

pub enum TgitError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl std::fmt::Display for TgitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &TgitError::IoError(ref e) => std::fmt::Debug::fmt(&e, formatter), // TODO: Understand this line of code, 1.Why the reference check 2.What's up with the ref keyword
            &TgitError::NoDirectory => formatter.write_str("No Directory Found"),
            &TgitError::InvalidCommit => formatter.write_str("The commit is invalid"),
            &TgitError::InvalidIndex => formatter.write_str("The index is corrupt"),
        }
    }
}

impl From<io::Error> for TgitError {
    fn from(err: io::Error) -> TgitError {
        TgitError::IoError(err)
    }
}
