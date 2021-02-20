use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {}

impl Error {
    pub fn new() -> Self {
        Error {}
    }
}

impl de::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for Error {}
