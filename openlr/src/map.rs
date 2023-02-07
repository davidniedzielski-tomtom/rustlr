use crate::edge::Edge;
use crate::errors::OpenLrErr;
use ::async_trait::async_trait;
use geo::Coord;

#[async_trait]
pub trait Map: Sync + Send {
    async fn get_nearby_edges(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr>;

    /// Returns a set of lines which follows this line in the same direction. The set of lines
    /// is equal to the set of outgoing lines of the end node of this line.
    async fn get_next_edges(
        &self,
        id: i64,
        meta: String,
    ) -> Result<Vec<Edge>, OpenLrErr>;
}
