use std::collections::HashMap;

use crate::astar::find_acceptable_shortest_path;
use crate::candidate_edge::CandidateEdge;
use crate::decoding_parameters::DecodingParameters;
use crate::edge::Edge;
use crate::request_context::RequestContext;

pub const DISTANCE_PER_SECTION: f64 = 15_000.0 / 256.0;
pub const DEGREES_PER_SECTION: f64 = 360.0 / 32.0;

pub(crate) async fn find_location_route(
    context: &RequestContext<'_, DecodingParameters>,
    candidates: &Vec<CandidateEdge<'_>>,
    cache: &mut HashMap<(i64, i64), Option<Vec<Edge>>>,
) -> Option<Vec<Vec<Edge>>> {
    let mut location_path: Vec<Vec<Edge>> = vec![];
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
                    for (index, src_id, dest_id) in cache_work {
                        cache.insert((src_id, dest_id), Some(location_path[index].clone()));
                    }
                    cache.insert(cache_key, None);
                    return None;
                }
            },
        }
    }

    Some(location_path)
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

pub fn calculate_circular_delta(v1: u16, v2: u16, sectors: u16) -> Option<u16> {
    if sectors % 2 != 0 || v1 >= sectors || v2 >= sectors {
        None
    } else {
        Some(match v1.abs_diff(v2) {
            d if d > sectors / 2 => sectors - d,
            d => d,
        })
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
        assert_eq!(calculate_circular_delta(360, 359, 32), None);
        assert_eq!(calculate_circular_delta(359, 360, 32), None);
        assert_eq!(calculate_circular_delta(359, 359, 361), None);
        assert_eq!(calculate_circular_delta(359, 359, 360).unwrap(), 0);
        assert_eq!(calculate_circular_delta(359, 0, 360).unwrap(), 1);
        assert_eq!(calculate_circular_delta(0, 359, 360).unwrap(), 1);
        assert_eq!(calculate_circular_delta(359, 1, 360).unwrap(), 2);
        assert_eq!(calculate_circular_delta(1, 359, 360).unwrap(), 2);
        assert_eq!(calculate_circular_delta(0, 180, 360).unwrap(), 180);
        assert_eq!(calculate_circular_delta(180, 0, 360).unwrap(), 180);
        assert_eq!(calculate_circular_delta(270, 90, 360).unwrap(), 180);
        assert_eq!(calculate_circular_delta(90, 270, 360).unwrap(), 180);

        assert_eq!(calculate_circular_delta(32, 31, 32), None);
        assert_eq!(calculate_circular_delta(31, 32, 32), None);
        assert_eq!(calculate_circular_delta(31, 31, 33), None);
        assert_eq!(calculate_circular_delta(31, 31, 32).unwrap(), 0);
        assert_eq!(calculate_circular_delta(31, 0, 32).unwrap(), 1);
        assert_eq!(calculate_circular_delta(0, 31, 32).unwrap(), 1);
        assert_eq!(calculate_circular_delta(31, 1, 32).unwrap(), 2);
        assert_eq!(calculate_circular_delta(1, 31, 32).unwrap(), 2);
        assert_eq!(calculate_circular_delta(0, 16, 32).unwrap(), 16);
        assert_eq!(calculate_circular_delta(16, 0, 32).unwrap(), 16);
        assert_eq!(calculate_circular_delta(24, 8, 32).unwrap(), 16);
        assert_eq!(calculate_circular_delta(8, 24, 32).unwrap(), 16);
    }
}
