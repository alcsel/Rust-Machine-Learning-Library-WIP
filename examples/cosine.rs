use rust_machine_learning::similarity_algorithms::cosine_similarity::cosine_similarity;
use std::io;

fn main() {
    // ---------------------------------------------------------
    // 1. READ FIRST VECTOR
    // ---------------------------------------------------------
    println!("Write the first vector's coordinates separated by space (e.g., 1.0 2.0 3.0):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    
    // Parse the input string into a vector of f64
    let v1: Vec<f64> = input1
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // ---------------------------------------------------------
    // 2. READ SECOND VECTOR
    // ---------------------------------------------------------
    println!("Write the second vector's coordinates separated by space (e.g., 4.0 5.0 6.0):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    
    // Parse the input string into a vector of f64
    let v2: Vec<f64> = input2
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // ---------------------------------------------------------
    // CALCULATION
    // ---------------------------------------------------------
    // Check if the lengths are equal
    if v1.len() != v2.len() {
        println!("Error: Vectors must have the same dimension!");
        return;
    }
    
    if v1.is_empty() {
        println!("Error: Vectors cannot be empty!");
        return;
    }

    // Calculate the cosine similarity between two vectors
    let similarity = cosine_similarity(&v1, &v2);

    println!("Vector 1: {:?}", v1);
    println!("Vector 2: {:?}", v2);
    println!("Cosine Similarity: {}", similarity);
}
//shitty ass algorithm

//cos(theta) = (v1 . v2) / (||v1|| * ||v2||)
