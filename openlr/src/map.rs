use crate::edge::Edge;
use crate::errors::OpenLrErr;
use ::async_trait::async_trait;
use geo::Coord;

#[async_trait]
pub trait Map: Sync {
    async fn get_lines_near_coordinates(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr>;

    /// Returns a set of lines which follows this line in the same direction. The set of lines
    /// is equal to the set of outgoing lines of the end node of this line.
    async fn get_next_lines(
        &self,
        src_edge_id: i64,
        src_meta: String,
    ) -> Result<Vec<Edge>, OpenLrErr>;
}
