use crate::binary_header::BinaryHeader;
use crate::common::{calculate_offset, find_route_across_lrps, get_next_coordinate, int2deg, trim};
use crate::decodable_reference::DecodableReference;
use crate::decoding_parameters::DecodingParameters;
use crate::deserializable_reference::DeserializableReference;
use crate::edge::Edge;
use crate::encodable_location::EncodableLocation;
use crate::encoding_parameters::EncodingParameters;
use crate::errors::OpenLrErr;
use crate::location::Location;
use crate::location_reference::LocationReference;
use crate::location_reference_point::LocationReferencePoint;
use crate::request_context::RequestContext;
use crate::serializable_reference::SerializableReference;
use async_trait::async_trait;
use serde::Serialize;

//--------------------------------------------------------------------//
//                                                                    //
// OpenLR Line                                                        //
//                                                                    //
//--------------------------------------------------------------------//

#[derive(Debug)]
pub struct LineLocationReference {
    pub(crate) header: BinaryHeader,
    pub(crate) lrps: Vec<LocationReferencePoint>,
    pub pos_offset: Option<(u32, u32)>,
    pub neg_offset: Option<(u32, u32)>,
}

impl LineLocationReference {
    // Given a sequence of Edges that connect the LRPs in a LocRef, generate
    // a LineLocation
    fn build_location(
        &self,
        path: Vec<Edge>,
        seg_start_offset: u32,
        seg_end_offset: u32,
    ) -> Result<LineLocation, OpenLrErr> {
        // Calculate the offsets into the path by considering both the offsets from the segment
        // start / end to the first / last LRP, as well as the pos/neg offsets from the LRP to
        // the start/end of the location.
        let mut start_index: usize = 0;
        let mut end_index: usize = path.len() - 1;
        let mut start_offset: u32 = seg_start_offset
            + if self.pos_offset.is_some() {
                self.pos_offset.unwrap().0
            } else {
                0
            };
        let mut end_offset: u32 = seg_end_offset
            + if self.neg_offset.is_some() {
                self.neg_offset.unwrap().0
            } else {
                0
            };

        // See if the path can be trimmed at either end as a result of the offsets completely
        // spanning leading or trailing segments
        if start_offset > 0 {
            match trim(&mut path.iter(), start_offset) {
                Some((index, offset)) => {
                    start_index = index;
                    start_offset = offset;
                }
                None => return Err(OpenLrErr::PostiveOffsetTooLong),
            }
        }

        if end_offset > 0 {
            match trim(&mut path.iter().rev(), end_offset) {
                Some((index, offset)) => {
                    end_index -= index;
                    end_offset = offset;
                }
                None => return Err(OpenLrErr::NegativeOffsetTooLong),
            }
        }

        // Report the offsets to the caller, taking into consideration
        // any margin of error introduced by the binary encoding
        Ok(LineLocation {
            p_off: if self.pos_offset.is_some() {
                let (t0, t1) = self.pos_offset.unwrap();
                Some((start_offset, start_offset + t1 - t0))
            } else if start_offset > 0 {
                Some((start_offset, start_offset))
            } else {
                None
            },
            n_off: if self.neg_offset.is_some() {
                let (t0, t1) = self.neg_offset.unwrap();
                Some((end_offset, end_offset + t1 - t0))
            } else if end_offset > 0 {
                Some((end_offset, end_offset))
            } else {
                None
            },
            edges: path[start_index..end_index + 1].to_owned(),
        })
    }

    pub fn build_lrp(
        ba: &[u8],
        prev: Option<&LocationReferencePoint>,
        index: usize,
        last: bool,
    ) -> LocationReferencePoint {
        let offset = if prev.is_none() { 6 } else { 4 };
        LocationReferencePoint::new_from_byte_array(
            match prev {
                None => int2deg(ba[0], ba[1], ba[2]),
                Some(lrp) => get_next_coordinate(ba[0], ba[1], lrp.longitude),
            },
            match prev {
                None => int2deg(ba[3], ba[4], ba[5]),
                Some(lrp) => get_next_coordinate(ba[2], ba[3], lrp.latitude),
            },
            &ba[offset..],
            index,
            last,
        )
    }
}

#[async_trait]
impl DecodableReference for LineLocationReference {
    type Peer = LineLocation;
    async fn decode(
        &self,
        context: &RequestContext<DecodingParameters>,
    ) -> Result<Location, OpenLrErr> {
        match find_route_across_lrps(&self.lrps, context).await {
            Ok((lp, pos_offset, neg_offset)) => {
                // we've found a satisfactory route: record the start/end offsets based on the start/end candidate
                // unwrap is safe because star will either fail or else return a path with at least one edge
                Ok(Location::Line(
                    self.build_location(lp, pos_offset, neg_offset).unwrap(),
                ))
            }
            Err(e) => Err(e),
        }
    }
}

impl DeserializableReference for LineLocationReference {
    type T = LineLocationReference;
    fn from_binary(bytes: &Vec<u8>) -> Result<Self::T, OpenLrErr> {
        let header = BinaryHeader::new(bytes[0]);

        let num_lrps = bytes.len() / 7;
        let num_offsets = (bytes.len() - 16) % 7;
        if num_offsets > 2 {
            return Err(OpenLrErr::InvalidBinaryStringLength(bytes.len()));
        }
        let mut lrps = Vec::new();
        let mut offset = 10;

        // Parse the first LocationReferencePoint
        lrps.push(LineLocationReference::build_lrp(
            &bytes[1..offset],
            None,
            0,
            false,
        ));

        // Parse the intermediate LRPSs
        for i in 1..num_lrps - 1 {
            lrps.push(LineLocationReference::build_lrp(
                &bytes[offset..offset + 7],
                Some(&lrps[i - 1]),
                i,
                false,
            ));
            offset += 7;
        }

        // Parse the last LRP
        lrps.push(LineLocationReference::build_lrp(
            &bytes[offset..offset + 6],
            Some(&lrps[num_lrps - 2]),
            num_lrps - 1,
            true,
        ));

        let (p_off, n_off) = (
            bytes[offset + 5] & 0b01000000 > 0,
            bytes[offset + 5] & 0b00100000 > 0,
        );

        // Determine if positive offset is present
        let pos_offset: Option<(u32, u32)> = if p_off {
            if offset + 6 >= bytes.len() {
                return Err(OpenLrErr::InvalidBinaryStringLength(bytes.len()));
            }
            Some(crate::common::calculate_offset(
                bytes[offset + 6],
                lrps[0].dnp.unwrap(),
            ))
        } else {
            None
        };

        // Determine if negative offset is present
        let neg_offset: Option<(u32, u32)> = if n_off {
            let adj = if p_off { 1 } else { 0 };
            if offset + 6 + adj >= bytes.len() {
                return Err(OpenLrErr::InvalidBinaryStringLength(bytes.len()));
            }
            Some(calculate_offset(
                bytes[offset + 6 + adj],
                lrps[num_lrps - 2].dnp.unwrap(),
            ))
        } else {
            None
        };

        // Return the LocationReference
        Ok(LineLocationReference {
            header,
            lrps,
            pos_offset,
            neg_offset,
        })
    }

    fn from_xml(xml: &str) -> Result<Self::T, OpenLrErr> {
        todo!()
    }
}

impl SerializableReference for LineLocationReference {
    fn to_binary(&self) -> Result<Vec<u8>, OpenLrErr> {
        todo!()
    }
    fn to_xml(&self) -> Result<String, OpenLrErr> {
        todo!()
    }
}

#[derive(Serialize, Debug)]
pub struct LineLocation {
    pub edges: Vec<Edge>,
    pub p_off: Option<(u32, u32)>,
    pub n_off: Option<(u32, u32)>,
}

#[async_trait]
impl EncodableLocation for LineLocation {
    type Peer = LineLocationReference;
    async fn encode(
        &self,
        context: &RequestContext<EncodingParameters>,
    ) -> Result<LocationReference, OpenLrErr> {
        todo!()
    }
}
