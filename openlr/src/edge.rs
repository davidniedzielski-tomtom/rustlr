use crate::errors::OpenLrErr;
use crate::fow::FOW;
use crate::frc::FRC;
use geo::{
    Bearing, Closest, ClosestPoint, Coord, CoordsIter, Geometry, HaversineDestination,
    HaversineDistance, LineInterpolatePoint, LineLocatePoint, LineString, Point,
};
use log::warn;
use serde::ser::SerializeStruct;
use serde::Serialize;
use wkt::{geo_types_from_wkt, TryFromWkt};

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: i64,
    pub meta: String,
    pub fow: FOW,
    pub frc: FRC,
    pub start_node: i64,
    pub end_node: i64,
    pub len: u32,
    pub geom: LineString,
}

impl Edge {
    /// Returns the start node of the line.
    pub(crate) fn get_start_node_id(&self) -> i64 {
        self.start_node
    }

    /// Returns the end node of the line.
    pub(crate) fn get_end_node_id(&self) -> i64 {
        self.end_node
    }

    /// Returns the start point of the line.
    pub(crate) fn get_start_point(&self) -> Point<f64> {
        self.geom.points().nth(1).unwrap()
    }

    /// Returns the end point of the line.
    pub(crate) fn get_end_point(&self) -> Point<f64> {
        self.geom
            .points()
            .nth(self.geom.coords_count() - 1)
            .unwrap()
    }

    /// Returns the line's FOW
    pub(crate) fn get_fow(&self) -> FOW {
        self.fow
    }

    /// Return the line's FRC
    pub(crate) fn get_frc(&self) -> FRC {
        self.frc
    }

    /// Gets a point along the line geometry which is {@code distance_along} meter
    /// away from the start node of the line. If the given distance exceeds the
    /// length of the line the end node is returned! The x-coordinate of the
    /// point refers to the longitude value and the y-coordinate refers to the
    /// latitude value.
    pub(crate) fn get_point_along_line(&self, distance_along: u32) -> Point<f64> {
        let fraction = distance_along as f64 / self.len as f64;
        self.geom.line_interpolate_point(fraction).unwrap()
    }

    /// Gets the length of the line indicating its real dimension along the geometry of the line. The resolution used
    /// for the length value should be meter [m].
    pub(crate) fn get_line_length(&self) -> u32 {
        self.len
    }

    /// Returns the line's unique ID.
    pub(crate) fn get_id(&self) -> i64 {
        self.id
    }

    /// Returns the edge's metadata
    pub(crate) fn get_metadata(&self) -> String {
        self.meta.to_owned()
    }

    /// Calculates the (smallest) distance in meters between the line and the
    /// point given by its longitude and latitude coordinates.
    pub(crate) fn distance_to_point(&self, longitude: f64, latitude: f64) -> u32 {
        let p1 = Point::new(longitude, latitude);
        let p2 = self.get_closest_point(p1);
        p1.haversine_distance(&p2) as u32
    }

    /// Calculates a projection point on the line for the given coordinates and
    /// returns the distance between the start node of the line and the
    /// projection point along the line shape. The projection point shall be that
    /// point on the line with the smallest distance between the line and the
    /// point given by the longitude and latitude coordinates.
    pub(crate) fn measure_along_line(&self, longitude: f64, latitude: f64) -> u32 {
        let p1 = Point::new(longitude, latitude);
        match self.geom.line_locate_point(&p1) {
            Some(frac) => (self.len as f64 * frac) as u32,
            _ => {
                warn!(
                    "Unable to determine location of point {:?} along edge {:?}.  Returning 0m",
                    p1, self.id
                );
                0 as u32
            }
        }
    }

    /// Calculate the bearing from an endpoint of the edge to a point located
    /// somewhere along the edge.  If `dir` is true, the bearing is from the
    /// starting point of the line to a point `offset` meters along the line.
    /// Otherwise, the bearing is measured from the end of the edge to a point
    /// len(edge) - `offset` meters along edge.
    pub(crate) fn bearing_to_point(&self, offset: u32, dir: bool) -> f64 {
        let p1: Point<f64>;
        let p2: Point<f64>;
        match dir {
            true => {
                p1 = self.get_start_point();
                p2 = if offset >= self.len as u32 {
                    self.get_end_point()
                } else {
                    self.get_point_along_line(offset)
                }
            }
            false => {
                p1 = self.get_end_point();
                p2 = if offset >= self.len as u32 {
                    self.get_start_point()
                } else {
                    self.get_point_along_line(self.len - offset)
                }
            }
        }
        let b = p1.bearing(p2);
        (b + 360.0) % 360.0
    }

    pub(crate) fn get_closest_point(&self, p1: Point<f64>) -> Point<f64> {
        match self.geom.closest_point(&p1) {
            Closest::Intersection(p) => p,
            Closest::SinglePoint(p) => p,
            _ => {
                warn!(
                        "closest point from {:?} to edge {:?} is indeterminate!  Returning start point...",
                        p1, self.id
                    );
                self.get_start_point()
            }
        }
    }

    /// Constructor that accepts pre-build geometry
    pub fn new(
        id: i64,
        meta: String,
        fow: FOW,
        frc: FRC,
        start_node: i64,
        end_node: i64,
        len: u32,
        geom: LineString,
    ) -> Self {
        Self {
            id,
            meta,
            fow,
            frc,
            start_node,
            end_node,
            len,
            geom,
        }
    }
    /// Constructor that build the geometry from a vector of Coords
    pub fn new_from_coords(
        id: i64,
        meta: String,
        fow: FOW,
        frc: FRC,
        start_node: i64,
        end_node: i64,
        len: u32,
        geom: Vec<Coord>,
    ) -> Self {
        Self {
            id,
            meta,
            fow,
            frc,
            start_node,
            end_node,
            len,
            geom: LineString::new(geom),
        }
    }
    /// Constructor that builds the geometry from WKT
    pub fn new_from_wkt(
        id: i64,
        meta: String,
        fow: FOW,
        frc: FRC,
        start_node: i64,
        end_node: i64,
        len: u32,
        geom: &str,
    ) -> Result<Self, OpenLrErr> {
        match LineString::try_from_wkt_str(geom) {
            Ok(g) => Ok(Self {
                id,
                meta,
                fow,
                frc,
                start_node,
                end_node,
                len,
                geom: g,
            }),
            _ => Err(OpenLrErr::InvalidEdgeWKT),
        }
    }
    /// Constructor that builds the Edge by reversing another edge and negates the id
    pub fn new_from_reverse(peer: &Self) -> Self {
        let mut rev_coords: Vec<&Coord> = peer.geom.coords().collect();
        rev_coords.reverse();
        let g = rev_coords
            .into_iter()
            .map(|e| e.clone())
            .collect::<Vec<Coord>>();
        //let g = rev_coords.iter().map(|e|*e.clone()).collect::<Vec<Coord>>();
        let g_r = LineString::new(g);
        Self {
            id: -peer.id,
            meta: peer.meta.clone(),
            fow: peer.fow,
            frc: peer.frc,
            start_node: peer.end_node,
            end_node: peer.start_node,
            len: peer.len,
            geom: g_r,
        }
    }
}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let coords = self
            .geom
            .coords()
            .map(|e| (e.x, e.y))
            .collect::<Vec<(f64, f64)>>();
        let mut state = serializer.serialize_struct("Edge", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("meta", &self.meta)?;
        state.serialize_field("len", &self.len)?;
        state.serialize_field("fow", &self.frc.to_usize())?;
        state.serialize_field("frc", &self.frc.to_usize())?;
        state.serialize_field("geom", &coords)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let e1 = Edge::new_from_wkt(
            8548148,
            "1152964797973692416".to_owned(),
            FOW::SingleCarriageway,
            FRC::FRC5,
            1111111,
            2222222,
            199,
            "LINESTRING(-0.42244 53.84129,-0.42241 53.84117,-0.4224 53.84109,-0.42235 53.84015,-0.42221 53.83952)").unwrap();
        let e2 = Edge::new_from_reverse(&e1);
        assert_eq!(e2.id, -e1.id);
        assert_eq!(e2.meta, e1.meta);
        assert_eq!(e2.fow, e1.fow);
        assert_eq!(e2.frc, e1.frc);
        assert_eq!(e2.start_node, e1.end_node);
        assert_eq!(e2.end_node, e1.start_node);
        assert_eq!(e2.len, e1.len);
        let e1_coords = e1.geom.coords().collect::<Vec<&Coord>>();
        let e2_coords = e2.geom.coords().collect::<Vec<&Coord>>();
        assert_eq!(e2_coords.first(), e1_coords.last());
        assert_eq!(e2_coords.last(), e1_coords.first());
    }
}
