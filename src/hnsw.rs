use crate::{
    point::Point, point_relation::PointRelation, search_layer::search_layer,
    select_neighbors::select_neighbors,
};
use rand::random;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HNSW {
    m: usize,
    layer_probas: Vec<f32>,
    ef_construction: usize,
    entry_point_id: String,
    points: HashMap<String, Point>,
}

impl HNSW {
    pub fn new(m: usize, ef_construction: usize) -> Self {
        let level_multipiler = 1.0 / (m as f32).ln();
        let points = HashMap::new();
        let layer_probas = HNSW::prepare_empty_layers(level_multipiler);
        return HNSW {
            layer_probas,
            m,
            ef_construction,
            entry_point_id: "".to_string(),
            points,
        };
    }

    pub fn insert(&mut self, id: &str, vector: &Vec<f32>) -> () {
        let mut point = self.create_point(id, vector);
        if self.entry_point_id == "" {
            self.entry_point_id = point.id();
            return;
        }
        let mut ep = self.entry_point();
        let mut layer = ep.layer_index();
        while layer > point.layer_index() {
            let layer_candidates = search_layer(&point, &ep, 1, layer, &self.points);
            let nearest_in_layer = layer_candidates.first().unwrap().clone();
            ep = self
                .points
                .get(&nearest_in_layer.neighbor_id())
                .unwrap()
                .clone();
            layer -= 1;
        }
        loop {
            let layer_candidates =
                search_layer(&point, &ep, self.ef_construction, layer, &self.points);
            let layer_neighbors =
                select_neighbors(&point, &layer_candidates, layer, &self.points, false, false);
            for neighbor in &layer_neighbors {
                let mut neighbor_point = self.points.get(&neighbor.neighbor_id()).unwrap().clone();
                neighbor_point.add_neighbor(&point);
                // FIXME: shrink layer neighbors
                neighbor_point.shrink_layer_neighbors(layer);
                self.points
                    .insert(neighbor_point.id(), neighbor_point.clone());
                point.add_neighbor(&neighbor_point);
                self.points.insert(point.id(), point.clone());
            }
            point.shrink_layer_neighbors(layer);
            let nearest_in_layer = layer_candidates.first().unwrap().clone();
            ep = self
                .points
                .get(&nearest_in_layer.neighbor_id())
                .unwrap()
                .clone();
            if layer == 0 {
                break;
            } else {
                layer -= 1;
            }
        }
    }

    pub fn knn_search(&self, q: &Vec<f32>, k: usize, ef: usize) -> Vec<PointRelation> {
        let mut ep = self.entry_point();
        let mut layer = ep.layer_index();
        let searching = Point::new("query", q.clone(), 0, self.m);
        while layer > 0 {
            let layer_candidates = search_layer(&searching, &ep, 1, layer, &self.points);
            let nearest_in_layer = layer_candidates.first().unwrap().clone();
            ep = self
                .points
                .get(&nearest_in_layer.neighbor_id())
                .unwrap()
                .clone();
            layer -= 1;
        }
        let result = search_layer(&searching, &ep, ef, layer, &self.points);
        let mut result: Vec<PointRelation> = result.into_iter().collect();
        if result.len() > k {
            result.truncate(k)
        }
        result
    }

    pub fn ef_construction(&self) -> usize {
        self.ef_construction
    }

    pub fn layer_probas(&self) -> Vec<f32> {
        self.layer_probas.clone()
    }

    pub fn points(&self) -> HashMap<String, Point> {
        self.points.clone()
    }

    fn entry_point(&self) -> Point {
        self.points.get(&self.entry_point_id).unwrap().clone()
    }

    fn select_point_layer(&self) -> usize {
        let mut f = random::<f32>();
        let mut index = 0;
        for proba in self.layer_probas.clone() {
            if f < proba {
                return index as usize;
            } else {
                f -= proba
            }
            index += 1;
        }
        return self.layer_probas.len() - 1;
    }

    fn prepare_empty_layers(m_l: f32) -> Vec<f32> {
        let mut index = 0;
        let mut layers: Vec<f32> = Vec::new();
        loop {
            let proba = (-1.0 * (index as f32) / m_l).exp() * (1.0 - (-1.0 / m_l).exp());
            if proba < 1e-9 {
                break;
            };
            layers.push(proba);
            index += 1
        }
        layers
    }

    fn create_point(&mut self, id: &str, vector: &Vec<f32>) -> Point {
        let layer = if self.points.len() == 0 {
            self.layer_probas.len() - 1
        } else {
            self.select_point_layer()
        };
        let point = Point::new(id, vector.clone(), layer, self.m);
        self.points.insert(id.to_string(), point.clone());
        point
    }
}
