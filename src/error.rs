use thiserror::Error;

/// All error definitions in gray-tree.
#[derive(Error, Debug)]
pub enum Error {
    /// Missing data field error.
    #[error("missing data field")]
    MissingDataField,
}

/// The result type.
pub type Result<T> = std::result::Result<T, Error>;
