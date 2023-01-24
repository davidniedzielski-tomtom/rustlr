use async_trait::async_trait;
use core::hash::Hash;
use openlr::edge::Edge;
use openlr::errors::OpenLrErr;
use openlr::fow::FOW;
use openlr::frc::FRC;
use openlr::map::Map;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::*;
use std::io::BufReader;

pub struct MockMap {
    edge_set: HashSet<Edge>,
}
impl MockMap {
    pub fn new() -> Self {
        MockMap {
            edge_set: HashSet::<Edge>::new(),
        }
    }
    pub fn new_from_csv(filename: &str) -> Self {
        let mut edge_set = HashSet::<Edge>::new();

        match File::open(filename) {
            Ok(file) => {
                let buf_reader = BufReader::new(file);

                for line in buf_reader.lines() {
                    let l = line.unwrap();
                    let v = l.split(":").collect::<Vec<&str>>();
                    let flowdir = v.get(4).unwrap().parse::<u8>().unwrap();
                    let e = Edge::new_from_wkt(
                        v.get(0).unwrap().parse::<i64>().unwrap(),
                        v.get(1).unwrap().trim_matches('\"').to_owned(),
                        FOW::from_u8(v.get(2).unwrap().parse::<u8>().unwrap()),
                        FRC::from_u8(v.get(3).unwrap().parse::<u8>().unwrap()),
                        v.get(5).unwrap().parse::<i64>().unwrap(),
                        v.get(6).unwrap().parse::<i64>().unwrap(),
                        v.get(7).unwrap().parse::<u32>().unwrap(),
                        v.get(8).unwrap().trim_matches('\"'),
                        false,
                    )
                    .unwrap();
                    if flowdir == 3 {
                        edge_set.insert(e);
                    } else if flowdir == 2 {
                        edge_set.insert(Edge::new_from_reverse(&e));
                    } else {
                        edge_set.insert(Edge::new_from_reverse(&e));
                        edge_set.insert(e);
                    }
                }
            }
            err => println!("Error: {:?}", err),
        }
        return Self { edge_set };
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
        println!(
            "radius search: lon: {}, lat: {}, radius: {}",
            lon, lat, radius
        );
        Ok(self
            .edge_set
            .iter()
            .filter(|e| e.distance_to_point(lon, lat) <= radius)
            .cloned()
            .collect::<Vec<Edge>>())
    }

    async fn get_next_lines(
        &self,
        src_edge_id: i64,
        src_node_id: i64,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        println!(
            "gen next: src_edge: {}, src_node: {}",
            src_edge_id, src_node_id
        );
        Ok(self
            .edge_set
            .iter()
            .filter(|e| e.start_node == src_node_id && e.id != -src_edge_id)
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
                let flowdir = v.get(4).unwrap().parse::<u8>().unwrap();
                let e = Edge::new_from_wkt(
                    v.get(0).unwrap().parse::<i64>().unwrap(),
                    v.get(1).unwrap().trim_matches('\"').to_owned(),
                    FOW::from_u8(v.get(2).unwrap().parse::<u8>().unwrap()),
                    FRC::from_u8(v.get(3).unwrap().parse::<u8>().unwrap()),
                    v.get(5).unwrap().parse::<i64>().unwrap(),
                    v.get(6).unwrap().parse::<i64>().unwrap(),
                    v.get(7).unwrap().parse::<u32>().unwrap(),
                    v.get(8).unwrap().trim_matches('\"'),
                    false,
                );
            }
        }
        err => println!("Error: {:?}", err),
    }
}
