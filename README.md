# Rust Machine Learning Library (WIP)

A collection of machine learning algorithms implemented in Rust, focused on clarity and performance.

## Algorithms

### Similarity
| Algorithm | Status |
|-----------|--------|
| Levenshtein Distance | ✅ |
| Cosine Similarity | ✅ |

## Usage

```toml
[dependencies]
rust_machine_learning = { git = "https://github.com/alcsel/Rust-Machine-Learning-Library-WIP" }
```

```rust
use rust_machine_learning::similarity_algorithms::levenstein_distance::levenshtein;
use rust_machine_learning::similarity_algorithms::cosine_similarity::cosine_similarity;

fn main() {
    // Levenshtein
    println!("Levenshtein distance: {}", levenshtein("kitten", "sitting")); // 3

    // Cosine Similarity
    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![4.0, 5.0, 6.0];
    println!("Cosine similarity: {}", cosine_similarity(&v1, &v2));
}
```

## Examples

You can run the interactive examples using Cargo:

```bash
cargo run --example levenstein
cargo run --example cosine
```

## Testing

```bash
cargo test
```

## License
MIT