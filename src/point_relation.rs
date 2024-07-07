use std::fmt;

use crate::point::Point;

pub struct PointRelation {
    self_id: String,
    neighbor_id: String,
    neighbor_layer: usize,
    distance: f32,
}

impl PointRelation {
    pub fn new(node: &Point, neighbor: &Point) -> Self {
        return PointRelation {
            self_id: node.id(),
            neighbor_id: neighbor.id(),
            neighbor_layer: neighbor.layer_index(),
            distance: PointRelation::euclidean_distance(&node.vector(), &neighbor.vector()),
        };
    }

    pub fn self_id(&self) -> String {
        return self.self_id.clone();
    }

    pub fn neighbor_id(&self) -> String {
        return self.neighbor_id.clone();
    }

    pub fn neighbor_layer(&self) -> usize {
        return self.neighbor_layer;
    }

    pub fn distance(&self) -> f32 {
        return self.distance;
    }

    fn euclidean_distance(v1: &[f32], v2: &[f32]) -> f32 {
        let mut sum = 0.0;
        for i in 0..v1.len() {
            sum += (v1[i] - v2[i]).powi(2);
        }

        sum.sqrt()
    }
}

impl Clone for PointRelation {
    fn clone(&self) -> Self {
        Self {
            self_id: self.self_id.clone(),
            neighbor_id: self.neighbor_id.clone(),
            neighbor_layer: self.neighbor_layer,
            distance: self.distance,
        }
    }
}

impl fmt::Debug for PointRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PointRelation(source={}, neighbor={}, neighbor_layer={}, distance={})",
            self.self_id, self.neighbor_id, self.neighbor_layer, self.distance
        )
    }
}

impl PartialEq for PointRelation {
    fn eq(&self, other: &Self) -> bool {
        return self.self_id == other.self_id && self.neighbor_id == other.neighbor_id;
    }
}

impl Eq for PointRelation {}

impl PartialOrd for PointRelation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.self_id != other.self_id {
            return None;
        }
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for PointRelation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.distance.partial_cmp(&other.distance).unwrap();
    }
}
