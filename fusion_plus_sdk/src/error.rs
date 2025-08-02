use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalError(String),
    InternalErrorStr(&'static str),
    UnsupportedChainId(u32),
    UnsupportedChainIdStr(String),
    MultichainAddressDecodeFailed(String),
    NetworkNameNotRecognised(String),
    Reqwest(Box<reqwest::Error>),
    SerdePathToError(Box<serde_path_to_error::Error<serde_json::Error>>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::InternalError(err)
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Self {
        Error::InternalErrorStr(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(Box::new(err))
    }
}
