#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidLayout { message: String },
    InvalidArgument { message: &'static str },
    Locked,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
