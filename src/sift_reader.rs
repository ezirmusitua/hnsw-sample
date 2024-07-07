use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_fvecs<P: AsRef<Path>>(path: P) -> Vec<Vec<f32>> {
    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read file");

    let mut vectors = Vec::new();
    let mut offset = 0;
    while offset < buffer.len() {
        let dim = u32::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]) as usize;
        offset += 4;
        let mut vector = Vec::with_capacity(dim);
        for _ in 0..dim {
            vector.push(f32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]));
            offset += 4;
        }
        vectors.push(vector);
    }
    vectors
}

pub fn read_ivecs<P: AsRef<Path>>(path: P) -> Vec<Vec<i32>> {
    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read file");

    let mut vectors = Vec::new();
    let mut offset = 0;
    while offset < buffer.len() {
        let dim = u32::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]) as usize;
        offset += 4;
        let mut vector = Vec::with_capacity(dim);
        for _ in 0..dim {
            vector.push(i32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]));
            offset += 4;
        }
        vectors.push(vector);
    }
    vectors
}
