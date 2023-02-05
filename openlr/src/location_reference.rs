use serde::Serialize;

use crate::line::LineLocationReference;

#[derive(Debug, Serialize)]
pub enum LocationReference {
    Line(LineLocationReference),
}

impl LocationReference {}
