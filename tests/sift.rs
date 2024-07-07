use hnsw_rs::{
    hnsw::HNSW,
    sift_reader::{read_fvecs, read_ivecs},
    utils::TimeIt,
};
extern crate hnsw_rs;

const NAME: &str = "siftsmall";

#[test]
fn test_hnsw_with_sift() {
    let mut timeit = TimeIt::new();
    let base_vectors = read_fvecs(format!("data/{NAME}_base.fvecs"));
    let queries = read_fvecs(format!("data/{NAME}_query.fvecs"));
    let ground_truth = read_ivecs(format!("data/{NAME}_groundtruth.ivecs"));
    timeit.print("load_sift");
    timeit.restart();
    let mut hnsw = HNSW::new(64, 1000);
    for (index, vector) in base_vectors.iter().enumerate() {
        hnsw.insert(&index.to_string(), vector);
    }
    timeit.print("build_index");
    let mut total_recall = 0.0;
    timeit.restart();
    for (query_index, query_vector) in queries.iter().enumerate() {
        let results = hnsw.knn_search(query_vector, 100, 1000);
        let truth = &ground_truth[query_index];
        let mut retrieved = 0;
        for candidate in &results {
            let index = candidate.neighbor_id().parse::<i32>().unwrap();
            if truth.contains(&index) {
                retrieved += 1;
            }
        }
        let recall = retrieved as f32 / results.len() as f32;
        total_recall += recall;
    }
    timeit.print("search");
    let average_recall = total_recall / queries.len() as f32;
    println!("average recall: {:.2}", average_recall);
    assert_eq!(average_recall >= 1.0, true);
}
