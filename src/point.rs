use std::collections::BTreeSet;

use crate::point_relation::PointRelation;

#[derive(Clone, Debug)]
pub struct Point {
    id: String,
    layer_index: usize,
    mmax: usize,
    vector: Vec<f32>,
    neighbors: BTreeSet<PointRelation>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Point {
    pub fn new(id: &str, vector: Vec<f32>, layer_index: usize, m: usize) -> Self {
        let mmax = if layer_index == 0 { 2 * m } else { m };
        return Point {
            id: id.to_string(),
            layer_index,
            mmax,
            neighbors: BTreeSet::new(),
            vector,
        };
    }

    pub fn id(&self) -> String {
        return self.id.clone();
    }

    pub fn layer_index(&self) -> usize {
        return self.layer_index;
    }

    pub fn mmax(&self) -> usize {
        self.mmax
    }

    pub fn neighbors(&self) -> Vec<PointRelation> {
        self.neighbors.clone().into_iter().collect()
    }

    pub fn vector(&self) -> Vec<f32> {
        return self.vector.clone();
    }

    pub fn add_neighbor(&mut self, node: &Point) {
        self.neighbors.insert(PointRelation::new(&self, node));
    }

    pub fn neighbors_in(&self, layer: usize) -> Vec<PointRelation> {
        self.neighbors()
            .iter()
            .filter(|n| n.neighbor_layer() == layer)
            .map(|n| n.clone())
            .collect()
    }

    pub fn shrink_layer_neighbors(&mut self, lc: usize) -> () {
        let mut lc_neighbors = self.neighbors_in(lc);
        while lc_neighbors.len() > self.mmax {
            let to_remove = lc_neighbors.pop().unwrap();
            self.neighbors.remove(&to_remove);
        }
    }
}
