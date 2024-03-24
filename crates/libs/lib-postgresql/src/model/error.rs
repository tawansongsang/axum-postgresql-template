use lib_auth::pwd;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::store;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, derive_more::From, Serialize)]
pub enum Error {
    UserIdNotFound,
    DataNotFoundFromCreated,
    UpdateError(String),

    // -- Modules
    #[from]
    Store(store::Error),
    #[from]
    Pwd(pwd::Error),
    UserInfo(QueryError),

    // -- Externals
    #[from]
    SqlX(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    Uuid(#[serde_as(as = "DisplayFromStr")] uuid::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize)]
pub enum QueryError {
    DataNotFound,
    NotReturnIDFromCreated,
    UserInfoRecordNotFound,
    NoTokenSalt,
}
