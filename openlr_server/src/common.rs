use geo::{LineString, Coord};
use openlr::{fow::FOW, frc::FRC, edge::Edge};
use crate::openlr_services::Coordinate;


pub fn proto_fow_from_fow(fow: &FOW) -> i32 {
    match fow {
        FOW::Undefined => 0,
        FOW::Motorway => 1,
        FOW::MultipleCarriageway => 2,
        FOW::SingleCarriageway => 3,
        FOW::Roundabout => 4,
        FOW::TrafficSquare => 5,
        FOW::SlipRoad => 6,
        FOW::Other => 7,
    }
}

pub fn proto_frc_from_frc(frc: &FRC) -> i32 {
    match frc {
        FRC::FRC0 => 0,
        FRC::FRC1 => 1,
        FRC::FRC2 => 2,
        FRC::FRC3 => 3,
        FRC::FRC4 => 4,
        FRC::FRC5 => 5,
        FRC::FRC6 => 6,
        FRC::FRC7 => 7,
    }
}

pub fn proto_edge_from_edge(e: &Edge) -> crate::openlr_services::Edge {
    crate::openlr_services::Edge {
        id: e.id,
        meta: e.meta.clone(),
        fow: proto_fow_from_fow(&e.fow),
        frc: proto_frc_from_frc(&e.frc),
        len: e.len,
        coords: e
            .geom
            .coords()
            .cloned()
            .map(|c| Coordinate {
                longitude: c.x,
                latitude: c.y,
            })
            .collect::<Vec<Coordinate>>(),
    }
}


pub fn edge_from_proto_edge(e: &crate::openlr_services::Edge) -> Edge {
    Edge {
        id: e.id,
        meta: e.meta.clone(),
        fow: FOW::from_u8(e.fow as u8),
        frc: FRC::from_u8(e.frc as u8),
        len: e.len,
        geom: LineString(
            e.coords
                .iter()
                .map(|c| Coord {
                    x: c.longitude,
                    y: c.latitude,
                })
                .collect::<Vec<Coord>>(),
        ),
    }
}