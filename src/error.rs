#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("db_error")]
    DbError(clickhouse::error::Error),
    #[error("not_found")]
    NotFound,
    #[error("alteady_exists")]
    AlreadyExists,
    #[error("unauthorized")]
    Unauthorized,
    #[error("cannot_delete_self")]
    CannotDeleteSelf,
}
impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<clickhouse::error::Error> for Error {
    fn from(err: clickhouse::error::Error) -> Self {
        if matches!(err, clickhouse::error::Error::RowNotFound) {
            Self::NotFound
        } else {
            Self::DbError(err)
        }
    }
}

impl From<Error> for poem::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Unauthorized => poem::Error::new(err, poem::http::StatusCode::UNAUTHORIZED),
            _ => poem::Error::new(err, poem::http::StatusCode::FORBIDDEN),
        }
    }
}

pub trait ErrorChecker {
    fn is_not_found(&self) -> bool;
}
impl<T> ErrorChecker for std::result::Result<T, Error> {
    fn is_not_found(&self) -> bool {
        match self {
            Ok(_) => false,
            Err(err) => err.is_not_found(),
        }
    }
}
