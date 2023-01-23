mod mock_map;

use openlr;
use openlr::fow::FOW;
use openlr::frc::FRC;
use openlr::log::LogLevel;
use openlr::decoding_parameters::DecodingParameters;
use mock_map::{RadiusSearchKey, MockMap};
use futures::executor::block_on;
use geo::{Coord, LineString};

#[test]
fn test_mockmap() {
    let e1 = openlr::edge::Edge {
        id: 8548148,
        meta: "1152964797973692416".to_owned(),
        len: 199,
        start_node: 6566423,
        end_node: 6566423,
        fow: FOW::SingleCarriageway,
        frc: FRC::FRC5,
        geom: LineString::new(vec![
            Coord {
                x: -0.42244,
                y: 53.84129,
            },
            Coord {
                x: -0.42241,
                y: 53.84117,
            },
            Coord {
                x: -0.42240,
                y: 53.84109,
            },
            Coord {
                x: -0.42235,
                y: 53.84015,
            },
            Coord {
                x: -0.42221,
                y: 53.83952,
            },
        ]),
    };
    let e2 = openlr::edge::Edge {
        id: 6882819,
        meta: "2305043733128019968".to_owned(),
        len: 199,
        start_node: 413245,
        end_node: 6162807,
        fow: FOW::SingleCarriageway,
        frc: FRC::FRC5,
        geom: LineString::new(vec![
            Coord {
                x: -0.42221,
                y: 53.83952,
            },
            Coord {
                x: -0.42211,
                y: 53.83904,
            },
        ]),
    };
    let mut map = MockMap::new();
    map.radius_search.insert(RadiusSearchKey((-0.42244791984558105, 53.84130120277405, 30)), vec!(e1.clone()));
    map.radius_search.insert(RadiusSearchKey((-0.42211791984558106, 53.83905120277405, 30)), vec!(e2.clone()));
    map.successor_search.insert((8548148, 6566423), vec!(e2.clone()));
    
    let loc_ref = openlr::deserialize_binary("C/+zGCZJgyuvBAAh/x8rHw==").unwrap();
    let loc = block_on(openlr::decode(1, loc_ref, &map, &DecodingParameters::default(), LogLevel::Trace));
    println!("{:?}", loc);
    
}