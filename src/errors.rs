use std::fmt::{Display, Formatter};

use iced;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    MismatchedOS,
    #[allow(dead_code)]
    MissingSaveFile,
    LocalLowNotFound,
    FailedFileRemoval,
    #[allow(dead_code)]
    FailedFileCopy,
    IcedError(iced::Error),
    IOError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred.")
    }
}

impl From<iced::Error> for Error {
    fn from(value: iced::Error) -> Self {
        Error::IcedError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}
