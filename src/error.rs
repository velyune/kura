#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Corruption { message: String },
    InvalidLayout { message: String },
    InvalidArgument { message: &'static str },
    Locked,
    SequenceOverflow,
    EncodingLimitExceeded { message: String },
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
