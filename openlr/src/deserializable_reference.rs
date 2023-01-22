use crate::errors::OpenLrErr;

pub trait DeserializableReference {
    type T;
    fn from_binary(binary: &Vec<u8>) -> Result<Self::T, OpenLrErr>;
    fn from_xml(xml: &str) -> Result<Self::T, OpenLrErr>;
}
