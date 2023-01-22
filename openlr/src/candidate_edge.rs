use crate::edge::Edge;
use crate::location_reference_point::LocationReferencePoint;

#[derive(Debug, Clone)]
pub struct CandidateEdge<'a> {
    pub(crate) candidate: Edge,
    pub(crate) offset: u32,
    pub(crate) lrp: &'a LocationReferencePoint,
    pub(crate) score: f64,
}
