use std::collections::HashMap;

use crate::binary_header::BinaryHeader;
use crate::candidate_edge::CandidateEdge;
use crate::common::{build_lrp, calculate_offset, find_location_route, trim};
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
use crate::route_generator::RouteGenerator;
use crate::serializable_reference::SerializableReference;
use async_trait::async_trait;
use itertools::Itertools;
use serde::Serialize;

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
    fn build_location<'a>(
        &self,
        path: Vec<Vec<Edge>>,
        seg_start_offset: u32,
        seg_end_offset: u32,
    ) -> Result<LineLocation, OpenLrErr> {
        // Concatenate the segments connecting the LRPs into a single path, removing duplicates.
        let pathvec: Vec<Edge> =
            path.into_iter()
                .concat()
                .iter()
                .fold(vec![], |mut acc: Vec<Edge>, elem| {
                    if acc.is_empty() || acc.last().unwrap().get_id() != elem.get_id() {
                        acc.push(elem.to_owned());
                        acc
                    } else {
                        acc
                    }
                });

        // Calculate the offsets into the path by considering both the offsets from the segment
        // start / end to the first / last LRP, as well as the pos/neg offsets from the LRP to
        // the start/end of the location.
        let mut start_index: usize = 0;
        let mut end_index: usize = pathvec.len() - 1;
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
            match trim(&mut pathvec.iter(), start_offset) {
                Some((index, offset)) => {
                    start_index = index;
                    start_offset = offset;
                }
                None => return Err(OpenLrErr::PostiveOffsetTooLong),
            }
        }

        if end_offset > 0 {
            match trim(&mut pathvec.iter().rev(), end_offset) {
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
            edges: pathvec[start_index..end_index + 1].to_owned(),
        })
    }
}

#[async_trait]
impl DecodableReference for LineLocationReference {
    type Peer = LineLocation;
    async fn decode(
        &self,
        context: &RequestContext<DecodingParameters>,
    ) -> Result<Location, OpenLrErr> {
        // lrp_candidates is a vector of Candidate edges, one for each LRP
        let mut lrp_candidates: Vec<Vec<CandidateEdge>> = vec![];

        // find potential edge matches for each LRP
        for lrp in &self.lrps {
            lrp_candidates.push(lrp.find_candidate_edges(context).await?);
        }

        // location path is a vector of edges connecting a pair of LRPs
        let mut location_path: Vec<Vec<Edge>> = vec![];
        let mut lrp_start_offset: u32 = 0;
        let mut lrp_end_offset: u32 = 0;

        // the route generator creates pairs of candidate endpoints in the optimal order
        let rg = RouteGenerator::new(lrp_candidates);
        // some subpaths may be repeated, so we cache subpaths in this hash map
        let mut path_cache: HashMap<(i64, i64), Option<Vec<Edge>>> = HashMap::new();

        // generate a fixed number of candidate sequences, and for each, attempt to connect LRP candidate edges
        for candidate_sequence in rg.into_iter().take(context.params.max_routing_attempts) {
            match find_location_route(context, &candidate_sequence, &mut path_cache).await {
                Some(lp) => {
                    // we've found a satisfactory route: record the start/end offsets based on the start/end candidate
                    location_path = lp;
                    lrp_start_offset = candidate_sequence[0].offset;
                    lrp_end_offset = candidate_sequence[candidate_sequence.len() - 1].offset;
                    break;
                }
                _ => (),
            }
        }

        // check reason for loop termination
        if location_path.is_empty() {
            context.info(|| format!("Unable to find path for location reference {:?}", self));
            Err(OpenLrErr::NoPathFound)
        } else {
            Ok(Location::Line(
                self.build_location(location_path, lrp_start_offset, lrp_end_offset)
                    .unwrap(),
            ))
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
        lrps.push(build_lrp(&bytes[1..offset], None, 0, false));

        // Parse the intermediate LRPSs
        for i in 1..num_lrps - 1 {
            lrps.push(build_lrp(
                &bytes[offset..offset + 7],
                Some(&lrps[i - 1]),
                i,
                false,
            ));
            offset += 7;
        }

        // Parse the last LRP
        lrps.push(build_lrp(
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

        // Return the LRP
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
