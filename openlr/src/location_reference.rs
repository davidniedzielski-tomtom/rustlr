use serde::Serialize;

use crate::line::LineLocationReference;

#[derive(Debug, Serialize)]
pub enum LocationReference {
    Line(LineLocationReference),
    // TODO: Add support for more location reference types
}

impl LocationReference {}
