use async_trait::async_trait;
use core::hash::Hash;
use openlr::edge::Edge;
use openlr::errors::OpenLrErr;
use openlr::map::Map;
use std::collections::HashMap;
use std::hash::Hasher;

pub struct RadiusSearchKey(pub (f64, f64, u32));

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

pub struct MockMap {
    pub radius_search: HashMap<RadiusSearchKey, Vec<Edge>>,
    pub successor_search: HashMap<(i64, i64), Vec<Edge>>,
}
impl MockMap {
    pub fn new() -> Self {
        MockMap {
            radius_search: HashMap::new(),
            successor_search: HashMap::new(),
        }
    }
}

#[async_trait]
impl Map for MockMap {
    async fn get_lines_near_coordinate(
        &self,
        lon: f64,
        lat: f64,
        radius: u32,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        println!("radius search: lon: {}, lat: {}, radius: {}", lon, lat, radius);
        match self.radius_search.get(&RadiusSearchKey((lon, lat, radius))) {
            Some(v) => {
                println!("Found");
                Ok(v.clone())
            },
            _ => {
                println!("No match");
                Ok(Vec::<Edge>::new())
            },
        }
    }

    async fn get_next_lines(
        &self,
        src_edge_id: i64,
        src_node_id: i64,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        println!("gen next: src_edge: {}, src_node: {}", src_edge_id, src_node_id);
        match self.successor_search.get(&(src_edge_id, src_node_id)) {
            Some(v) => {
                println!("Found");
                Ok(v.clone())
            },
            _ => {
                println!("No match");
                Ok(Vec::<Edge>::new())
            },
        }
    }
}
