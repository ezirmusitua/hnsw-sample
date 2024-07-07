use std::collections::{BTreeSet, HashMap};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{point::Point, point_relation::PointRelation};

pub fn select_neighbors(
    q: &Point,
    candidates: &BTreeSet<PointRelation>,
    lc: usize,
    points: &HashMap<String, Point>,
    extend_candidates: bool,
    keep_pruned_connections: bool,
) -> BTreeSet<PointRelation> {
    let mut result: BTreeSet<PointRelation> = BTreeSet::new();
    let mut working: BTreeSet<PointRelation> = BTreeSet::new();
    working.extend(candidates.clone());
    if extend_candidates {
        let to_insert: Vec<Vec<PointRelation>> = candidates
            .into_par_iter()
            .map(|candidate| {
                let candidate_node = points.get(&candidate.neighbor_id()).unwrap();
                let inner_vec: Vec<PointRelation> = candidate_node
                    .neighbors_in(lc)
                    .into_par_iter()
                    .map(|neighbor| {
                        let neighbor_node = points.get(&neighbor.self_id()).unwrap();
                        PointRelation::new(&q, &neighbor_node)
                    })
                    .collect();
                inner_vec
            })
            .collect();
        let to_insert: Vec<PointRelation> = to_insert
            .into_iter()
            .flat_map(|inner_vec| inner_vec.into_iter())
            .collect();
        for q_neighbor_relation in to_insert {
            if !working.contains(&q_neighbor_relation) {
                working.insert(q_neighbor_relation);
            }
        }
    }
    let mut working_dropped: Vec<PointRelation> = Vec::new();

    while working.len() > 0 && result.len() < q.mmax() {
        let nearest_in_working = working.pop_first().unwrap();
        let nearest_in_result = result.first();
        if nearest_in_result.is_none()
            || nearest_in_working.distance() < nearest_in_result.unwrap().distance()
        {
            result.insert(nearest_in_working);
        } else {
            working_dropped.push(nearest_in_working);
        }
    }

    if keep_pruned_connections {
        working_dropped.reverse();
        while working_dropped.len() > 0 && result.len() < q.mmax() {
            let nearest_in_working_dropped = working_dropped.pop().unwrap();
            result.insert(nearest_in_working_dropped.clone());
        }
    }

    result
}
