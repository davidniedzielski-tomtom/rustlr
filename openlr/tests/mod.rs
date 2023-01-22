use async_trait::async_trait;
use core::hash::Hash;
use openlr::edge::Edge;
use openlr::errors::OpenLrErr;
use openlr::fow::FOW;
use openlr::frc::FRC;
use openlr::map::Map;
use std::collections::HashMap;
use std::hash::Hasher;

struct RadiusSearchKey((f64, f64, u32));

impl PartialEq<Self> for RadiusSearchKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 .0 == other.0 .0 && self.0 .1 == other.0 .1 && self.0 .2 == other.0 .2
    }
}
impl Eq for RadiusSearchKey {}

impl Hash for RadiusSearchKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0 .0 * 10000000000.0).round() as i64).hash(state);
        ((self.0 .1 * 10000000000.0).round() as i64).hash(state);
        self.0 .2.hash(state)
    }
}

struct MockMap {
    radius_search: HashMap<RadiusSearchKey, Vec<Edge>>,
    successor_search: HashMap<(i64, i64), Vec<Edge>>,
}

#[async_trait]
impl Map for MockMap {
    async fn get_lines_near_coordinate(
        &self,
        lon: f64,
        lat: f64,
        radius: u32,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        match self.radius_search.get(&RadiusSearchKey((lon, lat, radius))) {
            Some(v) => Ok(v.clone()),
            _ => Ok(Vec::<Edge>::new()),
        }
    }

    async fn get_next_lines(
        &self,
        src_edge_id: i64,
        src_node_id: i64,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        match self.successor_search.get(&(src_edge_id, src_node_id)) {
            Some(v) => Ok(v.clone()),
            _ => Ok(Vec::<Edge>::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use geo::{Coord, LineString};

    use super::*;

    #[test]
    fn test_mockmap() {
        let e1 = Edge {
            id: 1,
            meta: "Dave".to_owned(),
            len: 32,
            start_node: 1,
            end_node: 2,
            fow: FOW::MultipleCarriageway,
            frc: FRC::FRC0,
            geom: LineString::new(vec![
                Coord {
                    x: 123.45,
                    y: 23.45,
                },
                Coord {
                    x: 123.56,
                    y: 23.67,
                },
            ]),
        };
    }
}
