pub(crate) mod astar;
pub(crate) mod binary_header;
pub(crate) mod candidate_edge;
pub(crate) mod common;
pub(crate) mod decodable_reference;
pub mod decoding_parameters;
pub(crate) mod deserializable_reference;
pub mod edge;
// pub mod geojson;
pub(crate) mod encodable_location;
pub mod encoding_parameters;
pub mod errors;
pub mod fow;
pub mod frc;
pub mod line;
pub mod location;
pub mod location_reference;
pub(crate) mod location_reference_point;
pub(crate) mod location_type;
pub mod log;
pub mod map;
pub(crate) mod request_context;
pub mod request_result;
pub(crate) mod route_generator;
pub(crate) mod serializable_reference;

use std::time::{Duration, SystemTime};

use crate::decodable_reference::DecodableReference;
use crate::decoding_parameters::DecodingParameters;
use crate::encoding_parameters::EncodingParameters;
use crate::errors::OpenLrErr;
use crate::location::Location;
use crate::location_reference::LocationReference;
use crate::location_type::LocationType;
use crate::log::LogLevel;
use crate::map::Map;
use deserializable_reference::DeserializableReference;
use line::LineLocationReference;
use crate::log::LogEntry;
use request_context::RequestContext;
use request_result::RequestResult;

pub async fn decode(
    id: u64,
    locref: &LocationReference,
    map_server: &dyn Map,
    params: &DecodingParameters,
    level: LogLevel,
) -> RequestResult<Location> {
    let start_time = SystemTime::now();
    let context = RequestContext::<DecodingParameters>::new(map_server, params, level);
    let result = match locref {
        LocationReference::Line(line_loc_ref) => line_loc_ref.decode(&context).await,
        lr => Err(OpenLrErr::UnsupportedLocationTypeError(format!("{:?}", lr))),
    };

    let elapsed = SystemTime::now().duration_since(start_time).unwrap();
    RequestResult::new(id, result, elapsed, context.get_log())
}

pub async fn encode(
    loc: &Location,
    map_server: &dyn Map,
    params: &EncodingParameters,
    level: LogLevel,
) -> Result<LocationReference, OpenLrErr> {
    todo!()
}

pub fn deserialize_binary(bin: &str) -> Result<LocationReference, OpenLrErr> {
    let bytes: Vec<u8> = match base64::decode(bin) {
        Ok(b) => b,
        Err(de) => {
            return Err(OpenLrErr::Base64ParseError(
                bin.parse().unwrap(),
                de.to_string(),
            ));
        }
    };

    match LocationType::from(&bytes) {
        LocationType::Line => Ok(LocationReference::Line(
            LineLocationReference::from_binary(&bytes).unwrap(),
        )),
        LocationType::Unknown => Err(OpenLrErr::UnknownLocationTypeError),
        loc => Err(OpenLrErr::UnsupportedLocationTypeError(format!(
            "{:?}",
            loc
        ))),
    }
}

pub async fn decode_binary(
    bin: &str,
    id: u64,
    map_server: &dyn Map,
    params: &DecodingParameters,
    level: LogLevel,
) -> RequestResult<Location> {
    let locref = match deserialize_binary(bin) {
        Ok(l) => l,
        Err(e) => {
            return RequestResult::new(id, Err(e), Duration::new(0, 0), Vec::<LogEntry>::new())
        }
    };

    decode(id, &locref, map_server, params, level).await
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_clone() {}
}
