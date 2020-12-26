pub mod admin;
pub mod cosmetics;

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MIN_ROWS: u32 = 20;
pub const MAX_ROWS: u32 = 100;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Paging {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct RespData<T: Serialize> {
    pub total: usize,
    pub data: T,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("invalid username")]
    InvalidUserName,
    #[error("invalid credentials (password)")]
    InvalidCredentials,
    #[error("could not hash password")]
    EncryptError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no permission")]
    NoPermissionError,
}

#[derive(Debug)]
pub enum CommonStatus {
    Valid = 0,
    Invalid = 1,
}

impl fmt::Display for CommonStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CommonStatus::Valid => write!(f, "0"),
            CommonStatus::Invalid => write!(f, "1"),
        }
    }
}

// validate request content input
pub trait Validate {
    fn validate(&self) -> Result<(), anyhow::Error>;
}
