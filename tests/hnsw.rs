use hnsw_rs::{hnsw::HNSW, point::Point, point_relation::PointRelation, utils::TimeIt};
use rand::random;

extern crate hnsw_rs;

const TOP_K: usize = 10;
const VECTOR_SIZE: usize = 128;
const NODE_COUNT: usize = 1000;

fn rnd_vector(size: usize) -> Vec<f32> {
    let mut vector = Vec::with_capacity(size);
    for _ in 0..size {
        vector.push(random::<f32>());
    }
    vector
}

fn prepare_test_data(top_k: usize) -> (Vec<Vec<f32>>, Vec<f32>, Vec<PointRelation>) {
    let base_vectors: Vec<Vec<f32>> = (0..NODE_COUNT)
        .into_iter()
        .map(|_| rnd_vector(VECTOR_SIZE))
        .collect();
    let query = Point::new("query", rnd_vector(VECTOR_SIZE), 0, 0);
    let mut truth: Vec<PointRelation> = Vec::new();
    for (index, vector) in base_vectors.iter().enumerate() {
        truth.push(PointRelation::new(
            &query,
            &Point::new(&index.to_string(), vector.clone(), 0, 0),
        ))
    }
    truth.sort();
    truth.truncate(top_k);
    (base_vectors, query.vector(), truth)
}

#[test]
fn test_hnsw() {
    let mut total_recall = 0.0;
    let mut timeit = TimeIt::new();
    let total_round = 10;
    for round in 0..total_round {
        println!("========== ROUND {} ==========", round);
        let (base_vectors, query, truth) = prepare_test_data(TOP_K);
        let mut hnsw = HNSW::new(64, 100);
        timeit.restart();
        for (index, vector) in base_vectors.iter().enumerate() {
            hnsw.insert(&index.to_string(), vector);
        }
        timeit.print("build index");
        timeit.restart();
        let results = hnsw.knn_search(&query, TOP_K, 100);
        timeit.print("search");
        use rayon::prelude::*;
        let retrieved = results
            .par_iter()
            .filter(|&candidate| truth.contains(candidate))
            .count();
        let recall = retrieved as f32 / results.len() as f32;
        total_recall += recall;
    }
    let average_recall = total_recall / total_round as f32;
    timeit.print("total");
    println!("average recall：{:.2}", average_recall);
    assert_eq!(
        average_recall >= 0.8,
        true,
        "average recall {} more than or equal to {}",
        average_recall,
        0.8
    );
}
