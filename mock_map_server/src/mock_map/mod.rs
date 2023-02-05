use async_trait::async_trait;
use geo::Coord;
use openlr::edge::Edge;
use openlr::errors::OpenLrErr;
use openlr::fow::FOW;
use openlr::frc::FRC;
use openlr::map_server::MapServer;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct MapEntry {
    edge: Edge,
    start_node: i64,
    end_node: i64,
}
impl MapEntry {
    fn new(edge: Edge, start_node: i64, end_node: i64) -> Self {
        MapEntry {
            edge,
            start_node,
            end_node,
        }
    }
}

#[derive(Debug)]
pub struct MockMap {
    edge_map: HashMap<i64, MapEntry>,
}
impl MockMap {
    pub fn new() -> Self {
        MockMap {
            edge_map: HashMap::<i64, MapEntry>::new(),
        }
    }
    pub fn new_from_csv(filename: &str) -> Self {
        let mut edge_map = HashMap::<i64, MapEntry>::new();

        match File::open(filename) {
            Ok(file) => {
                let buf_reader = BufReader::new(file);

                for line in buf_reader.lines() {
                    let l = line.unwrap();
                    let v = l.split(":").collect::<Vec<&str>>();
                    let start_node = v.get(5).unwrap().parse::<i64>().unwrap();
                    let end_node = v.get(6).unwrap().parse::<i64>().unwrap();
                    let e = Edge::new_from_wkt(
                        v.get(0).unwrap().parse::<i64>().unwrap(),
                        v.get(1).unwrap().trim_matches('\"').to_owned(),
                        FOW::from_u8(v.get(2).unwrap().parse::<u8>().unwrap()),
                        FRC::from_u8(v.get(3).unwrap().parse::<u8>().unwrap()),
                        v.get(7).unwrap().parse::<u32>().unwrap(),
                        v.get(8).unwrap().trim_matches('\"'),
                    )
                    .unwrap();
                    edge_map.insert(e.get_id(), MapEntry::new(e, start_node, end_node));
                }
            }
            err => println!("Error: {:?}", err),
        }
        return Self { edge_map };
    }
}

#[async_trait]
impl MapServer for MockMap {
    async fn get_lines_near_coordinates(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr> {
        Ok(points
            .iter()
            .map(|c| {
                self.edge_map
                    .values()
                    .filter(|me| me.edge.distance_to_point(c.x, c.y) <= radius)
                    .map(|me| me.edge.to_owned())
                    .collect::<Vec<Edge>>()
            })
            .collect::<Vec<Vec<Edge>>>())
    }

    async fn get_next_lines(
        &self,
        src_edge_id: i64,
        src_meta: String,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        println!(
            "next_line: src_edge: {}, src_meta: {}",
            src_edge_id, src_meta
        );
        let src = self.edge_map.get(&src_edge_id).unwrap();
        Ok(self
            .edge_map
            .values()
            .filter(|me| src.end_node == me.start_node && src.start_node != me.end_node)
            .map(|me| &me.edge)
            .cloned()
            .collect::<Vec<Edge>>())
    }
}

#[test]
fn test_csv() {
    use openlr::edge::Edge;
    use openlr::fow::FOW;
    use openlr::frc::FRC;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    match File::open("test_data/test1.csv") {
        Ok(file) => {
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let l = line.unwrap();
                let v = l.split(":").collect::<Vec<&str>>();
                let e = Edge::new_from_wkt(
                    v.get(0).unwrap().parse::<i64>().unwrap(),
                    v.get(1).unwrap().trim_matches('\"').to_owned(),
                    FOW::from_u8(v.get(2).unwrap().parse::<u8>().unwrap()),
                    FRC::from_u8(v.get(3).unwrap().parse::<u8>().unwrap()),
                    v.get(7).unwrap().parse::<u32>().unwrap(),
                    v.get(8).unwrap().trim_matches('\"'),
                );
            }
        }
        err => println!("Error: {:?}", err),
    }
}
