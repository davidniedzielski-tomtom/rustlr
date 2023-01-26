use crate::candidate_edge::CandidateEdge;
use crate::common::{calculate_circular_delta, distance_to_next_lrp, int2bearing};
use crate::decoding_parameters::DecodingParameters;
use crate::edge::Edge;
use crate::errors::OpenLrErr;
use crate::fow::FOW;
use crate::frc::FRC;
use crate::request_context::RequestContext;
use crate::{common, frc};
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Serialize, Debug)]
pub struct LocationReferencePoint {
    pub(crate) longitude: f64,
    pub(crate) latitude: f64,
    pub(crate) fow: FOW,
    pub(crate) frc: FRC,
    pub(crate) lowest_frc_to_next_point: Option<FRC>,
    pub(crate) bearing: (f64, f64),
    pub(crate) bearing_sector: u16,
    pub(crate) dnp: Option<(u32, u32)>,
    pub(crate) index: usize,
    pub(crate) is_last: bool,
}

impl LocationReferencePoint {
    // build an LRP given a lon/lat and a byte array or two or three bytes
    pub fn new_from_byte_array(
        longitude: f64,
        latitude: f64,
        ba: &[u8],
        index: usize,
        is_last: bool,
    ) -> Self {
        let bearing = int2bearing(ba[1] & 0b00011111);
        LocationReferencePoint {
            longitude,
            latitude,
            fow: FOW::from_u8(ba[0] & 0b00000111),
            frc: FRC::from_u8((ba[0] >> 3) & 0b00000111),
            lowest_frc_to_next_point: if !is_last {
                Some(FRC::from_u8(ba[1] >> 5))
            } else {
                None
            },
            bearing,
            bearing_sector: if bearing.0 != bearing.1 {
                ((bearing.0 + bearing.1) / (2.0 * common::DEGREES_PER_SECTION)) as u16
            } else {
                0
            },
            dnp: if !is_last {
                Some(distance_to_next_lrp(ba[2]))
            } else {
                None
            },
            index,
            is_last,
        }
    }

    fn score_candidate_edge(
        &self,
        candidate: &Edge,
        context: &RequestContext<'_, DecodingParameters>,
    ) -> f64 {
        //------------------------------//
        //  Bearing component           //
        //------------------------------//

        // get the bearing from edge start/end to measurement point along edge
        let bearing = candidate.bearing_to_point(context.params.bearing_distance, !self.is_last);

        let bearing_score = if self.bearing.0 == self.bearing.1 {
            // The bearing is a single value (i.e. derived from parsing XML). The
            // bearing component raw score is calculated by taking the absolute value of
            // the difference between the LRP's and edge's bearings, normalized to 180
            // degrees.
            let delta = f64::abs(self.bearing.0 - bearing);
            let delta = if delta <= 180.0 { delta } else { 360.0 - delta };
            delta / 180.0
        } else {
            // The bearing is within a range, sp determine how many "wedges" separate
            // the real and expected bearings.  That delta is used as an index into the
            // params.bearing_score_table to obtain the raw score
            let edge_sector = (bearing / common::DEGREES_PER_SECTION) as u16;
            let index = common::calculate_circular_delta(self.bearing_sector, edge_sector, 32)
                .unwrap() as usize;
            context.params.bearing_score_table[index] * context.params.bearing_weight
        };

        //------------------------------//
        //  FOW component               //
        //------------------------------//

        // Use the edge's and LRP's FOWs as indices into the params.fow_score_table to determine
        // the raw FOW score.
        let fow_score =
            context.params.fow_score_table[self.fow.to_usize()][candidate.get_fow().to_usize()];

        //------------------------------//
        //  FRC component               //
        //------------------------------//

        // Use the edge's and LRP's FRCs as indices into the params.fow_score_table to determine
        // the raw FRC score.
        let frc_score =
            context.params.frc_score_table[self.frc.to_usize()][candidate.get_frc().to_usize()];

        //------------------------------//
        //  Distance from LRP component //
        //------------------------------//

        // The distance from LRP raw score is the actual distance from the edge to the LRP normalized to
        // the search radius.
        let distance_score = candidate.distance_to_point(self.longitude, self.latitude) as f64;
        let distance_score = f64::min(1.0, distance_score / context.params.search_radius as f64);

        let edge_score = bearing_score * context.params.bearing_weight
            + fow_score * context.params.fow_weight
            + frc_score * context.params.frc_weight
            + distance_score * context.params.distance_weight;

        context.debug(|| format!("Candidate {} for LRP {} score: {} (bearing: {} * {}, fow: {} * {}, frc: {} * {}, distance: {} * {})",
            candidate.get_id(), self.index, edge_score,
            bearing_score, context.params.bearing_weight,
            fow_score, context.params.fow_weight,
            frc_score, context.params.frc_weight,
            distance_score, context.params.distance_weight,
        ));

        edge_score
    }

    // determine where to place this LRP on the candidate: at an endpoint or in the
    // interior of the candidate edge.  Set the offset based on whether this LRP is
    // the first, an intermediate, or the last LRP in the location.
    fn get_candidate_offset(
        &self,
        edge: &Edge,
        context: &RequestContext<'_, DecodingParameters>,
    ) -> u32 {
        // get the distance from start of line to the projection of this LRP
        let mut d = edge.measure_along_line(self.longitude, self.latitude);

        // if this is the last LRP, the offset is we're interested in the the distance
        // from the end of the line to the projection point
        if self.is_last {
            d = edge.get_line_length() - d
        }

        // if the projection point is sufficiently "close" to the appropriate end of
        // the line, snap the projection point to the line's start/end point. Otherwise,
        // snap this LRP onto a point in the interior of the edge.
        if (d > context.params.absolute_snapping_threshold)
            || ((d as f64 / edge.get_line_length() as f64)
                > context.params.relative_snapping_threshold)
        {
            d
        } else {
            if self.is_last {
                context.debug(|| {
                    format!(
                    "projection of LRP {} onto edge {} snapped to end node due to proximity ({}m)",
                    self.index,
                    edge.get_id(),
                    d
                )
                });
                0
            } else {
                context.debug(|| format!(
                    "projection of LRP {} onto edge {} snapped to start node due to proximity ({}m)", 
                    self.index,
                    edge.get_id(),
                    d
                ));
                0
            }
        }
    }

    // Query the supplied MapDatabase to find all edges that are a candidate for this LRP
    // given the constraints in the supplied derefercing parameter set
    pub(crate) async fn find_candidate_edges<'a>(
        &'a self,
        context: &RequestContext<'_, DecodingParameters>,
    ) -> Result<Vec<CandidateEdge<'a>>, OpenLrErr> {
        // find an unordered set of candidates that meet our requirements
        let mut candidates: Vec<CandidateEdge> = context
            .map
            .get_lines_near_coordinate(self.longitude, self.latitude, context.params.search_radius)
            .await?
            .into_iter()
            .map(|e: Edge| {
                let score = self.score_candidate_edge(&e, context);
                let offset = self.get_candidate_offset(&e, context);

                CandidateEdge {
                    candidate: e,
                    offset,
                    lrp: self,
                    score,
                }
            })
            .filter(|ce: &CandidateEdge| ce.score <= context.params.max_acceptable_rating)
            .collect::<Vec<CandidateEdge>>();

        if candidates.is_empty() {
            // no eligible candidates could be found, so inform the caller
            Err(OpenLrErr::NoCandidatesFoundForLRP(self.index))
        } else {
            // sort the candidate list by score
            if candidates.len() > 1 {
                candidates.sort_by(|a, b| {
                    let t = a.score.partial_cmp(&b.score);

                    // All else being equal, prefer the candidate with the smallest offset
                    match t {
                        Some(Ordering::Equal) if a.offset < b.offset => Ordering::Less,
                        Some(_) => t.unwrap(),
                        _ => {
                            context.warn(|| {
                                format!(
                                    " Comparison of {:?} and {:?} failed.  Returning equal",
                                    a, b
                                )
                            });
                            Ordering::Equal
                        }
                    }
                })
            }
            // return no more than max_candidates_per_lrp candidates
            candidates.truncate(context.params.max_candidates_per_lrp);
            context.debug(|| {
                format!(
                    "Candidates for lrp {}: {:?}",
                    self.index,
                    candidates
                        .iter()
                        .map(|c| c.candidate.get_id())
                        .collect::<Vec<i64>>()
                )
            });
            Ok(candidates)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_from_byte_array() {
        let ba: &[u8] = &[0xf1, 0xcf, 0xff];
        let lrp = LocationReferencePoint::new_from_byte_array(1.999, 2.999, ba, 0, false);
        assert_eq!(lrp.longitude, 1.999);
        assert_eq!(lrp.latitude, 2.999);
        assert_eq!(lrp.frc, FRC::FRC6);
        assert_eq!(lrp.fow, FOW::Motorway);
        assert_eq!(lrp.bearing, (168.75, 180.0));
        assert_eq!(lrp.dnp, Some((14941, 15000)));
        assert_eq!(lrp.lowest_frc_to_next_point, Some(FRC::FRC6));

        let ba: &[u8] = &[0xf1, 0xcf];
        let lrp = LocationReferencePoint::new_from_byte_array(1.999, 2.999, ba, 0, true);
        assert_eq!(lrp.longitude, 1.999);
        assert_eq!(lrp.latitude, 2.999);
        assert_eq!(lrp.frc, FRC::FRC6);
        assert_eq!(lrp.fow, FOW::Motorway);
        assert_eq!(lrp.bearing, (168.75, 180.0));
        assert_eq!(lrp.dnp, None);
        assert_eq!(lrp.lowest_frc_to_next_point, None);
    }
}
