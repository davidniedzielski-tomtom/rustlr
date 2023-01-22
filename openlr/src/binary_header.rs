use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug)]
pub struct BinaryHeader {
    header_byte: u8,
}

#[derive(Serialize)]
pub enum AreaFlag {
    CircleOrNoAreaLocation,
    Polygon,
    RectangleOrGrid,
    ClosedLine,
}

impl<'a> Serialize for BinaryHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("OpenlrHeader", 4)?;
        state.serialize_field("version", &self.version())?;
        state.serialize_field("isPoint", &self.is_point())?;
        state.serialize_field("areaFlags", &self.area_flag())?;
        state.serialize_field("hasAttrs", &self.has_attrs())?;
        state.end()
    }
}

impl BinaryHeader {
    pub fn new(b: u8) -> Self {
        BinaryHeader { header_byte: b }
    }

    pub fn has_attrs(&self) -> bool {
        self.header_byte & 0b00001000 > 0
    }

    pub fn area_flag(&self) -> AreaFlag {
        match ((self.header_byte >> 5) & 0b00000010) + ((self.header_byte >> 4) & 0b00000001) {
            0 => AreaFlag::CircleOrNoAreaLocation,
            1 => AreaFlag::Polygon,
            2 => AreaFlag::RectangleOrGrid,
            3 => AreaFlag::ClosedLine,
            _ => unreachable!(),
        }
    }

    pub fn version(&self) -> u8 {
        self.header_byte & 0b00000111
    }

    pub fn is_point(&self) -> bool {
        self.header_byte & 0b00100000 > 0
    }
}
