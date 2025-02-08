//! Error used for database.

use anyhow::Result;
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Path points to directory")]
    IsDirectory,

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}
