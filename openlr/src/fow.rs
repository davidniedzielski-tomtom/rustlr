use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum FOW {
    Undefined = 0,
    Motorway = 1,
    MultipleCarriageway = 2,
    SingleCarriageway = 3,
    Roundabout = 4,
    TrafficSquare = 5,
    SlipRoad = 6,
    Other = 7,
}

impl Copy for FOW {}

impl FOW {
    #[allow(dead_code)]
    pub fn to_usize(&self) -> usize {
        match self {
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
    pub fn from_u8(i: u8) -> Self {
        match i {
            0 => FOW::Undefined,
            1 => FOW::Motorway,
            2 => FOW::MultipleCarriageway,
            3 => FOW::SingleCarriageway,
            4 => FOW::Roundabout,
            5 => FOW::TrafficSquare,
            6 => FOW::SlipRoad,
            7 => FOW::Other,
            _ => unreachable!(),
        }
    }
}
