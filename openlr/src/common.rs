use std::collections::{HashMap, VecDeque};

use geo::Coord;
use itertools::Itertools;

use crate::astar::find_acceptable_shortest_path;
use crate::candidate_edge::CandidateEdge;
use crate::decoding_parameters::DecodingParameters;
use crate::edge::Edge;
use crate::errors::OpenLrErr;
use crate::location_reference_point::LocationReferencePoint;
use crate::request_context::RequestContext;
use crate::route_generator::RouteGenerator;

pub const DISTANCE_PER_SECTION: f64 = 15_000.0 / 256.0;
pub const DEGREES_PER_SECTION: f64 = 360.0 / 32.0;

pub(crate) async fn find_route_across_lrps<'a>(
    lrps: &'a Vec<LocationReferencePoint>,
    context: &'a RequestContext<'a, DecodingParameters>,
) -> Result<(Vec<Edge>, u32, u32), OpenLrErr> {
    let lrp_candidates = find_candidates(lrps, context).await?;

    // the route generator creates pairs of candidate endpoints in the optimal order
    let rg = RouteGenerator::new(&lrp_candidates);
    // some subpaths may be repeated across generatied sequences, so we cache subpaths in this hash map
    let mut path_cache: HashMap<(i64, i64), Option<Vec<Edge>>> = HashMap::new();

    // generate a fixed number of candidate sequences, and for each, attempt to connect LRP candidate edges
    for candidate_indices in rg.into_iter().take(context.params.max_routing_attempts) {
        let candidate_sequence = candidate_indices
            .iter()
            .enumerate()
            .map(|(lrp_index, candidate_index)| {
                lrp_candidates
                    .get(lrp_index)
                    .unwrap()
                    .get(*candidate_index)
                    .unwrap()
            })
            .collect::<Vec<&CandidateEdge>>();
        match find_location_route(context, &candidate_sequence, &mut path_cache).await {
            Some(lp) => {
                context.debug(|| {
                    format!(
                        "Path found: {:?}",
                        lp.iter().map(|e| e.get_id()).collect::<Vec<i64>>()
                    )
                });
                return Ok((
                    lp,
                    candidate_sequence.first().unwrap().offset,
                    candidate_sequence.last().unwrap().offset,
                ));
            }
            _ => {
                context.debug(|| {
                    format!(
                        "Unable to find path for candidate sequence: {:?}",
                        candidate_sequence
                            .iter()
                            .map(|c| c.candidate.get_id())
                            .collect::<Vec<i64>>()
                    )
                });
            }
        }
    }
    return Err(OpenLrErr::NoPathFound);
}

pub(crate) async fn find_candidates<'a>(
    lrps: &'a Vec<LocationReferencePoint>,
    context: &'a RequestContext<'a, DecodingParameters>,
) -> Result<Vec<Vec<CandidateEdge<'a>>>, OpenLrErr> {

    // for each lrp, gather a vector of edges that are spatially near the point
    // do all lrps at once to minimize the latency to the mapserver
    let mut nearby_edges = VecDeque::from(
        context
            .map_server
            .get_nearby_edges(
                lrps.iter()
                    .map(|lrp| Coord {
                        x: lrp.longitude,
                        y: lrp.latitude,
                    })
                    .collect::<Vec<Coord>>(),
                context.params.search_radius,
            )
            .await?,
    );

    let mut candidates = Vec::<Vec<CandidateEdge>>::new();

    // if the caller is curious, record the edges near each lrp
    if context.is_enabled_for_debug() {
        for (i, v) in nearby_edges.iter().enumerate() {
            let eids = v.iter().map(|e| e.id).collect::<Vec<i64>>();
            context.debug(|| format!("Edges near LRP {} (lon: {}, lat: {}): {:?}", i, lrps[i].longitude, lrps[i].latitude, eids));
        }
    }

    // evaulate each candidate for each lrp, and select only those whose score is acceptable
    for lrp in lrps {
        let v = lrp.find_candidate_edges(nearby_edges.pop_front().unwrap(), context);
        match v {
            Ok(v) => {
                if context.is_enabled_for_debug() {
                    let cids = v.iter().map(|c| (c.candidate.id, c.offset, c.score)).collect::<Vec<(i64, u32, f64)>>();
                    context.debug(|| format!("Scored candidates (id, offset, score) for LRP {}: {:?}", lrp.index, cids));
                }
                candidates.push(v)
            },
            Err(e) => return Err(e),
        }
    }
    return Ok(candidates);
}

pub(crate) async fn find_location_route(
    context: &RequestContext<'_, DecodingParameters>,
    candidates: &Vec<&CandidateEdge<'_>>,
    cache: &mut HashMap<(i64, i64), Option<Vec<Edge>>>,
) -> Option<Vec<Edge>> {
    let mut location_path: Vec<Vec<Edge>> = vec![];
    // As we attempt to find paths between adjacent pairs
    // of edges, we accumulate the results cheaply in
    // this Vec.  If we can find a path spanning the entire
    // location, we make no updates to the caller's cache,
    // because we won't be called again with another sequence
    // of edges.  Otherwise, we (expensively) clone results
    // in the work cache into the caller's cache, because
    // if we're called again, we can reuse previous search results.
    let mut cache_work: Vec<(usize, i64, i64)> = vec![];

    // for each consecutive pair of candidate edges, see if we can find an acceptable route linking the corresponding LRPs.
    for pair in candidates.windows(2) {
        let cache_key = (pair[0].candidate.get_id(), pair[1].candidate.get_id());
        context.debug(|| {
            format!(
                "Attemping to find subpath between LRP {} ({}) and LRP {} ({})",
                pair[0].lrp.index, cache_key.0, pair[1].lrp.index, cache_key.1
            )
        });
        match cache.get(&cache_key) {
            Some(None) => {
                context.trace(|| {
                    format!(
                        "Shortest path search avoided for pair: {:?} (None)",
                        cache_key
                    )
                });
                return None;
            }
            Some(Some(v)) => {
                context.trace(|| {
                    format!(
                        "Shortest path search avoided for pair: {:?} (Some)",
                        cache_key
                    )
                });
                location_path.push(v.clone())
            }
            _ => match find_acceptable_shortest_path(&pair[0], &pair[1], context).await {
                Ok(sp) => {
                    context.debug(|| {
                        format!(
                            "Path found between edges {:?}: {:?}",
                            cache_key,
                            sp.iter().map(|e| e.id).collect::<Vec<i64>>()
                        )
                    });
                    cache_work.push((location_path.len(), cache_key.0, cache_key.1));
                    location_path.push(sp);
                }
                Err(openlr_err) => {
                    context.debug(|| {
                        format!(
                            "Routing failed between edges {:?}: {:?}",
                            cache_key, openlr_err
                        )
                    });
                    // We failed to find a path across *all* adjacent pairs, but we may have
                    // successfully found subpaths.  Save our work into the caller's cache to
                    // be reused on subsequent calls.
                    for (index, src_id, dest_id) in cache_work {
                        cache.insert((src_id, dest_id), Some(location_path[index].clone()));
                    }
                    cache.insert(cache_key, None);
                    return None;
                }
            },
        }
    }

    // Concatenate the subpaths into a single vector of edges, removing adjacent duplicates along the way
    Some(
        location_path
            .into_iter()
            .concat()
            .iter()
            .fold(vec![], |mut acc: Vec<Edge>, elem| {
                if acc.is_empty() || acc.last().unwrap().get_id() != elem.get_id() {
                    acc.push(elem.to_owned());
                    acc
                } else {
                    acc
                }
            }),
    )
}

pub fn trim<'a>(it: &mut dyn Iterator<Item = &'a Edge>, offset: u32) -> Option<(usize, u32)> {
    let mut piter = it.peekable();
    let mut pathlen: u32 = 0;
    let mut index: usize = 0;

    loop {
        match piter.next_if(|e| pathlen + e.get_line_length() <= offset) {
            Some(x) => {
                pathlen += x.get_line_length();
                index += 1;
            }
            None => break,
        }
    }

    if piter.peek().is_none() {
        None
    } else {
        Some((index, offset - pathlen))
    }
}

pub fn calculate_circular_delta(v1: u16, v2: u16, sectors: u16) -> u16 {
    if v1 > v2 {
        u16::min(v1 - v2, v2 + sectors - v1)
    } else {
        u16::min(v2 - v1, v1 + sectors - v2)
    }
}

pub fn signum(i: f64) -> f64 {
    if i > 0.0 {
        0.5
    } else if i < 0.0 {
        -0.5
    } else {
        0.0
    }
}

pub fn distance_to_next_lrp(d: u8) -> (u32, u32) {
    let du32 = d as u32;
    (
        (du32 as f64 * DISTANCE_PER_SECTION).floor() as u32,
        ((du32 + 1) as f64 * DISTANCE_PER_SECTION).floor() as u32,
    )
}

pub fn int2bearing(sector: u8) -> (f64, f64) {
    let sf64 = sector as f64;
    (
        sf64 * DEGREES_PER_SECTION,
        (sf64 + 1.) * DEGREES_PER_SECTION,
    )
}

pub fn get_next_coordinate(ub: u8, lb: u8, prev: f64) -> f64 {
    let relative: i16 = (((ub as u16) << 8) | (lb as u16)) as i16;
    ((relative as f64) / 100_000.0) + prev
}

pub fn int2deg(f: u8, m: u8, l: u8) -> f64 {
    let f1 = u32::from(f);
    let m1 = u32::from(m);
    let l1 = u32::from(l);

    let j1 = (f1 << 16) | (m1 << 8) | l1;
    let i: f64 = (if j1 > (1 << 23) {
        let t: i32 = j1 as i32;
        t - (1 << 24)
    } else {
        j1 as i32
    }) as f64;

    ((i - signum(i)) * 360.0) / 16777216.0
}

pub fn calculate_offset(offset: u8, dnp: (u32, u32)) -> (u32, u32) {
    (
        ((offset as u32) * dnp.0) / 256,
        ((offset as u32) * dnp.1) / 256,
    )
}

#[cfg(test)]
mod tests {
    use super::calculate_circular_delta;
    #[test]
    fn test_calc_circular_delta() {
        assert_eq!(calculate_circular_delta(360, 359, 32), 1);
        assert_eq!(calculate_circular_delta(359, 360, 32), 1);
        assert_eq!(calculate_circular_delta(359, 359, 361), 0);
        assert_eq!(calculate_circular_delta(359, 359, 360), 0);
        assert_eq!(calculate_circular_delta(359, 0, 360), 1);
        assert_eq!(calculate_circular_delta(0, 359, 360), 1);
        assert_eq!(calculate_circular_delta(359, 1, 360), 2);
        assert_eq!(calculate_circular_delta(1, 359, 360), 2);
        assert_eq!(calculate_circular_delta(0, 180, 360), 180);
        assert_eq!(calculate_circular_delta(180, 0, 360), 180);
        assert_eq!(calculate_circular_delta(270, 90, 360), 180);
        assert_eq!(calculate_circular_delta(90, 270, 360), 180);

        assert_eq!(calculate_circular_delta(32, 31, 32), 1);
        assert_eq!(calculate_circular_delta(31, 32, 32), 1);
        assert_eq!(calculate_circular_delta(31, 31, 33), 0);
        assert_eq!(calculate_circular_delta(31, 31, 32), 0);
        assert_eq!(calculate_circular_delta(31, 0, 32), 1);
        assert_eq!(calculate_circular_delta(0, 31, 32), 1);
        assert_eq!(calculate_circular_delta(31, 1, 32), 2);
        assert_eq!(calculate_circular_delta(1, 31, 32), 2);
        assert_eq!(calculate_circular_delta(0, 16, 32), 16);
        assert_eq!(calculate_circular_delta(16, 0, 32), 16);
        assert_eq!(calculate_circular_delta(24, 8, 32), 16);
        assert_eq!(calculate_circular_delta(8, 24, 32), 16);
    }
}
