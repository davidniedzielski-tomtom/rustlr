use serde::Serialize;

use crate::line::LineLocation;

#[derive(Serialize, Debug)]
pub enum Location {
    Line(LineLocation),
    // TODO: Add support for more location types
    Unknown,
}
