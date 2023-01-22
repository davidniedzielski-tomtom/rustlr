pub(crate) mod astar;
pub(crate) mod binary_header;
pub(crate) mod candidate_edge;
pub(crate) mod common;
pub(crate) mod decodable_reference;
pub mod decoding_parameters;
pub(crate) mod deserializable_reference;
pub mod edge;
pub(crate) mod encodable_location;
pub mod encoding_parameters;
pub mod errors;
pub mod fow;
pub mod frc;
pub(crate) mod line;
pub(crate) mod location;
pub(crate) mod location_reference;
pub(crate) mod location_reference_point;
pub(crate) mod location_type;
pub mod log;
pub mod map;
pub(crate) mod request_context;
pub mod request_result;
pub(crate) mod route_generator;
pub(crate) mod serializable_reference;

use std::time::SystemTime;

use crate::decodable_reference::DecodableReference;
use crate::decoding_parameters::DecodingParameters;
use crate::encodable_location::EncodableLocation;
use crate::encoding_parameters::EncodingParameters;
use crate::errors::OpenLrErr;
use crate::location::Location;
use crate::location_reference::LocationReference;
use crate::location_type::LocationType;
use crate::log::{LogEntry, LogLevel};
use crate::map::Map;
use deserializable_reference::DeserializableReference;
use line::LineLocationReference;
use request_context::RequestContext;
use request_result::RequestResult;

pub async fn decode(
    id: i64,
    locref: LocationReference,
    map: &dyn Map,
    params: &DecodingParameters,
    level: LogLevel,
) -> RequestResult<Location> {
    let start_time = SystemTime::now();
    match locref {
        LocationReference::Line(line_loc_ref) => {
            let context = RequestContext::<DecodingParameters>::new(map, params, level);
            let res = line_loc_ref.decode(&context).await;
            match res {
                Ok(line_location) => RequestResult::new(
                    id,
                    Ok(Location::Line(line_location)),
                    SystemTime::now().duration_since(start_time).unwrap(),
                    context.get_log(),
                ),
                Err(err) => RequestResult::new(
                    id,
                    Err(err),
                    SystemTime::now().duration_since(start_time).unwrap(),
                    Vec::<LogEntry>::new(),
                ),
            }
        }
        _ => RequestResult::new(
            id,
            Err(OpenLrErr::UnknownLocationTypeError),
            SystemTime::now().duration_since(start_time).unwrap(),
            Vec::<LogEntry>::new(),
        ),
    }
}

pub async fn encode(
    loc: Location,
    map: &dyn Map,
    params: &EncodingParameters,
    level: LogLevel,
) -> Result<LocationReference, OpenLrErr> {
    match loc {
        Location::Line(loc) => {
            let context = RequestContext::<EncodingParameters>::new(map, params, level);
            Ok(LocationReference::Line(loc.encode(&context).await.unwrap()))
        }
        _ => Err(OpenLrErr::UnknownLocationTypeError),
    }
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
        _ => Err(OpenLrErr::UnknownLocationTypeError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
