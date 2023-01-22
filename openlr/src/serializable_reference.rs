use crate::errors::OpenLrErr;

pub trait SerializableReference {
    fn to_binary(&self) -> Result<Vec<u8>, OpenLrErr>;
    fn to_xml(&self) -> Result<String, OpenLrErr>;
}
