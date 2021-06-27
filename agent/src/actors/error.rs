use thiserror::Error;

#[derive(Debug, Error)]
#[error("inner value already taken")]
pub struct AlreadyTaken;
