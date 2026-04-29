// File: src/similarity_algorithms/cosine_similarity.rs

pub fn cosine_similarity(v1: &[f64], v2: &[f64]) -> f64 {
    // Return 0 if dimensions don't match or vectors are empty
    if v1.len() != v2.len() || v1.is_empty() {
        return 0.0; 
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    // Calculate dot product and squared norms in a single loop
    for i in 0..v1.len() {
        dot_product += v1[i] * v2[i];
        norm_a += v1[i] * v1[i];
        norm_b += v2[i] * v2[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0; // Prevent division by zero
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}
