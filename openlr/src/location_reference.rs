use crate::line::LineLocationReference;

#[derive(Debug)]
pub enum LocationReference {
    Line(LineLocationReference),
}

impl LocationReference {}
