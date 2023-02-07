use std::path::PathBuf;
use mock_map_agent::mock_map::MockMap;
use futures::executor::block_on;
use openlr;
use openlr::decoding_parameters::DecodingParameters;
use openlr::location::Location;
use openlr::log::LogLevel;
use serde::Serialize;

#[test]
fn test_decode1() {
    let map = MockMap::new_from_csv("test_data/test1.csv");

    let loc_ref = openlr::deserialize_binary("C/+zGCZJgyuvBAAh/x8rHw==").unwrap();
    let mut params = DecodingParameters::default();
    params.bearing_weight = 0.35;
    params.fow_weight = 0.2;
    params.frc_weight = 0.1;
    params.distance_weight = 0.35;
    let loc = block_on(openlr::decode(1, &loc_ref, &map, &params, LogLevel::Debug));
    println!("{:?}", loc);
    assert!(loc.result.is_ok());
    match loc.result {
        Ok(Location::Line(l)) => {
            assert_eq!(l.edges.len(), 2);
            assert!(l.p_off.is_none());
            assert!(l.n_off.is_none());
            assert_eq!(l.edges.get(0).unwrap().id, 8548148);
            assert_eq!(l.edges.get(1).unwrap().id, 6882819);
        }
        _ => {
            assert_eq!(1, 0);
        }
    }
}

#[test]
fn test_decode2() {
    let map = MockMap::new_from_csv("test_data/test2.csv");

    let loc_ref = openlr::deserialize_binary("C/5kUCVBsjPVAv8f/+QzBw==").unwrap();
    let loc = block_on(openlr::decode(
        1,
        &loc_ref,
        &map,
        &DecodingParameters::default(),
        LogLevel::Trace,
    ));
    assert!(loc.result.is_ok());
    match loc.result {
        Ok(Location::Line(l)) => {
            assert_eq!(l.edges.len(), 1);
            assert!(l.p_off.is_none());
            assert!(l.n_off.is_none());
            assert_eq!(l.edges.get(0).unwrap().id, 14183861);
        }
        _ => {
            assert_eq!(1, 0);
        }
    }
}

#[test]
fn test_decode3() {
    fn get_current_working_dir() -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }
    println!("{:?}", get_current_working_dir());
    let map = MockMap::new_from_csv("./test_data/test3.csv");

    let loc_ref = openlr::deserialize_binary("C/4bnSaa4yu5Af91ACAruQT+r/+9Kwc=").unwrap();
    let loc = block_on(openlr::decode(
        1,
        &loc_ref,
        &map,
        &DecodingParameters::default(),
        LogLevel::Debug,
    ));
    println!("{:?}", loc);
    assert!(loc.result.is_ok());
    match loc.result {
        Ok(Location::Line(l)) => {
            assert_eq!(l.edges.len(), 5);
            assert!(l.p_off.is_none());
            assert!(l.n_off.is_none());
            assert_eq!(l.edges.get(0).unwrap().id, 13776006);
            assert_eq!(l.edges.get(1).unwrap().id, 3480569);
            assert_eq!(l.edges.get(2).unwrap().id, 3064123);
            assert_eq!(l.edges.get(3).unwrap().id, 6529766);
            assert_eq!(l.edges.get(4).unwrap().id, 4617407);
        }
        _ => {
            assert_eq!(1, 0);
        }
    }
}

#[test]
fn test_decode4() {
    fn get_current_working_dir() -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }
    println!("{:?}", get_current_working_dir());
    let map = MockMap::new_from_csv("test_data/test4.csv");

    let loc_ref = openlr::deserialize_binary("C/7VOCaEbSu/BP+5AMUrbJEQ").unwrap();
    println!("{:?}", loc_ref);
    let loc = block_on(openlr::decode(
        1,
        &loc_ref,
        &map,
        &DecodingParameters::default(),
        LogLevel::Debug,
    ));
    println!("{:?}", loc);
    assert!(loc.result.is_ok());
    match loc.result {
        Ok(Location::Line(l)) => {
            assert_eq!(l.edges.len(), 4);
            assert_eq!(l.p_off, Some((31, 64)));
            assert_eq!(l.n_off, Some((15, 19)));
            assert_eq!(l.edges.get(0).unwrap().id, 11384678);
            assert_eq!(l.edges.get(1).unwrap().id, 3551999);
            assert_eq!(l.edges.get(2).unwrap().id, 12007589);
            assert_eq!(l.edges.get(3).unwrap().id, 13042191);
        }
        _ => {
            assert_eq!(1, 0);
        }
    }
}

#[test]
fn test_serialize_line_loc_ref() {
    let loc_ref = openlr::deserialize_binary("C/7VOCaEbSu/BP+5AMUrbJEQ").unwrap();
    println!("{:?}", serde_json::to_string(&loc_ref).unwrap())
}

#[test]
fn test_both() {
    fn get_current_working_dir() -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }
    println!("{:?}", get_current_working_dir());
    let map = MockMap::new_from_csv("test_data/test4.csv");
    let loc = block_on(openlr::decode_binary(
        "C/7VOCaEbSu/BP+5AMUrbJEQ",
        1,
        &map,
        &DecodingParameters::default(),
        LogLevel::Debug,
    ));
    println!("{:?}", loc);
    assert!(loc.result.is_ok());
    match loc.result {
        Ok(Location::Line(l)) => {
            assert_eq!(l.edges.len(), 4);
            assert_eq!(l.p_off, Some((31, 64)));
            assert_eq!(l.n_off, Some((15, 19)));
            assert_eq!(l.edges.get(0).unwrap().id, 11384678);
            assert_eq!(l.edges.get(1).unwrap().id, 3551999);
            assert_eq!(l.edges.get(2).unwrap().id, 12007589);
            assert_eq!(l.edges.get(3).unwrap().id, 13042191);
        }
        _ => {
            assert_eq!(1, 0);
        }
    }
}
