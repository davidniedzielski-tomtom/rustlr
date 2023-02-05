use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenLrServerErr {
    #[error("UrlParseError")]
    UrlParseError(#[from] url::ParseError),
    #[error("RekwestError")]
    RekwestError(#[from] reqwest::Error),
    #[error("MapDatabase creation error: {0}")]
    MapDatabaseCreationError(String),
    #[error("Implementation error: {0}")]
    ImplementationError(String),
    #[error("Unknown error encountered during OpenLR processing")]
    Unknown,
}
