use crate::candidate_edge::CandidateEdge;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct FWrap(f64);

impl Eq for FWrap {}

impl PartialOrd for FWrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for FWrap {
    fn cmp(&self, other: &FWrap) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct RouteGenerator<'a> {
    candidates: &'a Vec<Vec<CandidateEdge<'a>>>,
    heap: PriorityQueue<Vec<usize>, FWrap>,
}

impl<'a> RouteGenerator<'a> {
    pub fn new(candidates: &'a Vec<Vec<CandidateEdge<'a>>>) -> Self {
        let mut heap = PriorityQueue::new();
        // initialize the queue with the first candidate for each lrp (the best route)
        let i = candidates.iter().fold((vec![], 0f64), |mut acc, e| {
            acc.0.push(0usize);
            (acc.0, acc.1 + e.first().unwrap().score)
        });

        heap.push(i.0, FWrap(i.1));
        RouteGenerator { candidates, heap }
    }
}

impl<'a> Iterator for RouteGenerator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return the next best vector of candidate indices whose connected path
        // will span the location reference.

        // first, pop the leading candidate combination from the heap (i.e. the vector
        // of candidate indices per LRP ) with the optimum path weight.
        match self.heap.pop() {
            Some((indices, ..)) => {
                // Before returning this path, score and push all n permutations of it
                // onto the heap (by incrementing each of the n indices within it).

                // For each LRP, consider the next best candidate given the current best
                // combination
                for i in 0..indices.len() {
                    if indices[i] + 1 < self.candidates[i].len() {
                        let mut t = indices.clone();
                        t[i] += 1;
                        // calculate the score for the permutated route
                        let total = t
                            .iter()
                            .enumerate()
                            .fold(0f64, |acc, (k, j)| acc + self.candidates[k][*j].score);
                        self.heap.push(t, FWrap(total));
                    }
                }

                // Return this leading candidate after first resolving the indices to the candidate segments they
                // represent.
                Some(indices)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RouteGenerator;
    use crate::candidate_edge::CandidateEdge;

    // #[test]
    // fn test_rg_1() {
    //     let c1 = CandidateEdge{
    //         candidate: todo!(),
    //         offset: todo!(),
    //         lrp: todo!(),
    //         score: 123.456,
    //     };
    //     let c2 = CandidateEdge{
    //         candidate: todo!(),
    //         offset: todo!(),
    //         lrp: todo!(),
    //         score: 345.456,
    //     };

    //     let candidates = vec![
    //         vec![c1, c2]
    //     ];

    //     let rg = RouteGenerator::new(candidates);
    // }
    #[test]
    fn test_borrow() {
        struct Holder<'a> {
            v: &'a Vec<f64>,
        }

        impl<'a> Holder<'a> {
            fn get(&self) -> &f64 {
                &self.v[0]
            }
        }

        let v = vec![0.0f64, 1.0f64];
        let h = Holder { v: &v };
        println!("{:?}", h.get())
    }
}
