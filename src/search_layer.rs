use std::collections::{BTreeSet, HashMap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{point::Point, point_relation::PointRelation};

pub fn search_layer(
    q: &Point,
    ep: &Point,
    ef: usize,
    lc: usize,
    points: &HashMap<String, Point>,
) -> BTreeSet<PointRelation> {
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(ep.id());
    let q_ep_rel = PointRelation::new(&q, &ep);
    let mut candidates: BTreeSet<PointRelation> = BTreeSet::new();
    candidates.insert(q_ep_rel.clone());
    let mut working: BTreeSet<PointRelation> = BTreeSet::new();
    working.insert(q_ep_rel.clone());
    while candidates.len() > 0 {
        let candidate_nearest = candidates.pop_first().unwrap();
        let furtherest_in_working = working.last().unwrap().clone();
        if candidate_nearest.distance() > furtherest_in_working.distance() {
            break;
        }
        let candidate_nearest_node = points.get(&candidate_nearest.neighbor_id()).unwrap();
        let q_neighbor_relations: Vec<PointRelation> = candidate_nearest_node
            .neighbors_in(lc)
            .into_par_iter()
            .filter(|relation| !visited.contains(&relation.neighbor_id()))
            .map(|relation| {
                let neighbor_node = points.get(&relation.neighbor_id()).unwrap();
                PointRelation::new(&q, neighbor_node)
            })
            .collect();
        for q_neighbor_relation in q_neighbor_relations {
            visited.insert(q_neighbor_relation.neighbor_id());
            let furtherest_in_working = working.last().unwrap().clone();
            if q_neighbor_relation.distance() >= furtherest_in_working.distance()
                && working.len() >= ef
            {
                break;
            }
            candidates.insert(q_neighbor_relation.clone());
            working.insert(q_neighbor_relation.clone());
        }
    }
    working
}
