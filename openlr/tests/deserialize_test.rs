mod mock_map;

use futures::executor::block_on;
use geo::{Coord, LineString};
use mock_map::{MockMap, RadiusSearchKey};
use openlr;
use openlr::decoding_parameters::DecodingParameters;
use openlr::fow::FOW;
use openlr::frc::FRC;
use openlr::location::Location;
use openlr::log::LogLevel;

#[test]
fn test_mockmap() {
    let e1 = openlr::edge::Edge::new_from_wkt(
        8548148,
        "1152964797973692416".to_owned(),
        FOW::SingleCarriageway,
        FRC::FRC5,
        6566423,
        6566423,
        199,
        "LINESTRING(-0.42244 53.84129,-0.42241 53.84117,-0.4224 53.84109,-0.42235 53.84015,-0.42221 53.83952)").unwrap();

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
    map.radius_search.insert(
        RadiusSearchKey((-0.42244791984558105, 53.84130120277405, 30)),
        vec![e1.clone()],
    );
    map.radius_search.insert(
        RadiusSearchKey((-0.42211791984558106, 53.83905120277405, 30)),
        vec![e2.clone()],
    );
    map.successor_search
        .insert((8548148, 6566423), vec![e2.clone()]);

    let loc_ref = openlr::deserialize_binary("C/+zGCZJgyuvBAAh/x8rHw==").unwrap();
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
