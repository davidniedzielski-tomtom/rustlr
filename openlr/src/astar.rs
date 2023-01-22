use crate::candidate_edge::CandidateEdge;
use crate::decoding_parameters::DecodingParameters;
use crate::edge::Edge;
use crate::map::Map;
use crate::request_context::RequestContext;
use crate::OpenLrErr;
use crate::OpenLrErr::NoSubPathFound;
use indexmap::map::Entry::{Occupied, Vacant};
use indexmap::IndexMap;
use num_traits::Zero;
use rustc_hash::FxHasher;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::{BuildHasherDefault, Hash, Hasher};

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

fn reverse_path<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let path = itertools::unfold(start, |i| {
        parents.get_index(*i).map(|(node, value)| {
            *i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();
    // Collecting the going through the vector is needed to revert the path because the
    // unfold iterator is not double-ended due to its iterative nature.
    path.into_iter().rev().cloned().collect()
}

// EdgeWrapper is a newtype pattern around a map Edge to make it amenable to various trait
// implementations (like hashing, cloning, Ord, etc) necessary for the astar algorithm
struct EdgeWrapper(Edge);
impl PartialEq<Self> for EdgeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_id() == other.0.get_id()
    }
}
impl Eq for EdgeWrapper {}
impl Hash for EdgeWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get_id().hash(state)
    }
}
impl Clone for EdgeWrapper {
    fn clone(&self) -> Self {
        EdgeWrapper(self.0.clone())
    }
}

// SmallestCostHolder is copied unmodified from the pathfinder crate:
//
struct SmallestCostHolder {
    estimated_cost: u32,
    cost: u32,
    index: usize,
}

impl PartialEq for SmallestCostHolder {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl Eq for SmallestCostHolder {}

impl PartialOrd for SmallestCostHolder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SmallestCostHolder {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

pub(crate) async fn find_acceptable_shortest_path<'a>(
    src: &'a CandidateEdge<'a>,
    dst: &'a CandidateEdge<'a>,
    context: &RequestContext<'_, DecodingParameters>,
) -> Result<Vec<Edge>, OpenLrErr> {
    let dst_id = dst.candidate.get_id();
    let p = dst.candidate.get_start_point();
    let (dst_lon, dst_lat) = (p.x(), p.y());
    let max_acceptable_frc = context.params.allowed_frc_delta_table[src.lrp.frc.to_usize()];
    let src_dnp = src.lrp.dnp.unwrap().1;
    // set the maximum path length as the greater of the maximum absolute and relative deviations
    let max_distance = u32::max(
        src_dnp + context.params.max_absolute_dnp_variance,
        (src_dnp as f64 * (1.0 + context.params.max_relative_dnp_variance)) as u32,
    );

    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: Zero::zero(),
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<EdgeWrapper, (usize, u32)> = FxIndexMap::default();
    parents.insert(
        EdgeWrapper(src.candidate.clone()),
        (usize::MAX, Zero::zero()),
    );
    while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap();

            context.trace(|| {
                format!(
                    "Top candidate {} popped from heap with cost: {}",
                    node.0.get_id(),
                    cost
                )
            });

            if node.0.get_id() == dst_id {
                let path = reverse_path(&parents, |&(p, _)| p, index);
                return Ok(path.into_iter().map(|ew| ew.0).collect());
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }

            // Find all outgoing lines from the current line that satisfies LFRC constraints and
            // leaves the cumulative path length in the DNP Goldilocks zone (not too long).
            context
                .map
                .get_next_lines(node.0.get_id(), node.0.get_end_node_id())
                .await
                .unwrap()
                .into_iter()
                .filter(|e| e.get_frc().to_usize() <= max_acceptable_frc)
                .map(|n| {
                    let move_cost = if n.get_id() == dst_id {
                        0u32
                    } else {
                        n.get_line_length()
                    };
                    (EdgeWrapper(n), move_cost)
                })
        };

        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;

            // Only consider this successor if the path length hasn't exceeded the LRP DNP tolerance
            if new_cost < max_distance {
                let h; // heuristic(&successor)
                let n; // index for successor
                let id = successor.0.get_id();
                match parents.entry(successor) {
                    Vacant(e) => {
                        let edge_id = e.key().0.get_id();
                        h = if edge_id == dst_id {
                            0u32
                        } else {
                            e.key().0.distance_to_point(dst_lon, dst_lat)
                        };
                        n = e.index();
                        e.insert((index, new_cost));
                    }
                    Occupied(mut e) => {
                        let edge_id = e.key().0.get_id();
                        if e.get().1 > new_cost {
                            h = if edge_id == dst_id {
                                0u32
                            } else {
                                e.key().0.distance_to_point(dst_lon, dst_lat)
                            };
                            n = e.index();
                            e.insert((index, new_cost));
                        } else {
                            continue;
                        }
                    }
                }

                to_see.push(SmallestCostHolder {
                    estimated_cost: new_cost + h,
                    cost: new_cost,
                    index: n,
                });

                context.trace(|| format!("Successor {} added to heap with estimated cost: {} (initial: {} + move {} + h: {})", id, new_cost + h, cost, move_cost, h));
            } else {
                context.debug(|| {
                    format!(
                        "Successor {} rejected due to excessive path length {} ( > {} )",
                        successor.0.get_id(),
                        new_cost,
                        max_distance
                    )
                });
            }
        }
    }
    Err(NoSubPathFound(src.lrp.index, dst.lrp.index))
}
