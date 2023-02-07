use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum OpenLrErr {
    #[error("Invalid Edge WKT")]
    InvalidEdgeWKT,
    #[error("Unknown LocationType")]
    UnknownLocationTypeError,
    #[error("Unsupported LocationType: {0}")]
    UnsupportedLocationTypeError(String),
    #[error("Implementation error: {0}")]
    ImplementationError(String),
    #[error("Error retrieving edge {0}: {1}")]
    EdgeRetrievalError(i64, String),
    #[error("Error retrieving next lines for edge {0}: {1}")]
    NextLinesError(i64, String),
    #[error("Error performing nearby edges search: {0}")]
    NearbyEdgesError(String),
    #[error("Cannot build Location from empty Edge vector")]
    EmptyEdgeVec,
    #[error("Negative offset spans entire found path")]
    NegativeOffsetTooLong,
    #[error("Postive offset spans entire found path")]
    PostiveOffsetTooLong,
    #[error("Error accessing LRP FOW score row (index {0}) from dereferencing parameters")]
    InvalidFOWScoreRowIndex(usize),
    #[error("Error accessing edge FOW score element (index {0}) from dereferencing parameters")]
    InvalidFOWScoreColumnIndex(usize),
    #[error("Error accessing LRP FRC score row (index {0}) from dereferencing parameters")]
    InvalidFRCScoreRowIndex(usize),
    #[error("Error accessing edge FRC score element (index {0}) from dereferencing parameters")]
    InvalidFRCScoreColumnIndex(usize),
    #[error("Error accessing bearing score table index {0} from dereferencing parameters")]
    InvalidBearingScoreIndex(usize),
    #[error("Error calculating circular delta for bearing {0} and {1} (sectorsize: {2})")]
    InvalidBearingDelta(u16, u16, u16),
    #[error("No path connecting LRPs found")]
    NoPathFound,
    #[error(
        "Path between LRP {0} and LRP {2} found but was too long (actual: {1}, expected: {3})"
    )]
    PathLengthTooLong(usize, u16, usize, u16),
    #[error(
        "Path between LRP {0} and LRP {2} found but was too short (actual: {1}, expected: {3})"
    )]
    PathLengthTooShort(usize, u16, usize, u16),
    #[error("No subpath connecting LRPs {0} and {1} found")]
    NoSubPathFound(usize, usize),
    #[error("No edges near LRP {0} could be found")]
    NoEdgesNearLRP(usize),
    #[error("No candidates found for LRP {0}")]
    NoCandidatesFoundForLRP(usize),
    #[error("Unable to parse base64 string: {0}. Reason: {1}")]
    Base64ParseError(String, String),
    #[error("Cannot deserialize. Invalid base64 string length: {0}")]
    InvalidBinaryStringLength(usize),
    #[error("Error from radius search: {0}")]
    NextSearchError(String),
    #[error("Error from next edge search: {0}")]
    NextEdgeError(String),
    #[error("Unknown error encountered during OpenLR processing")]
    Unknown,
}
