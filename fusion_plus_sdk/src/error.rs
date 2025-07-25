pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalError(String),
    InternalErrorStr(&'static str),
    Reqwest(Box<reqwest::Error>),
    SerdePathToError(Box<serde_path_to_error::Error<serde_json::Error>>),
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
