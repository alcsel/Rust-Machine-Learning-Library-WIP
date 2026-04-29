# Rust Machine Learning Library (WIP)

A collection of machine learning algorithms implemented in Rust, focused on clarity and performance.

## Algorithms

### Similarity
| Algorithm | Status |
|-----------|--------|
| Levenshtein Distance | ✅ |
| Cosine Similarity | 🔜 |

## Usage

```toml
[dependencies]
rust_machine_learning = { git = "https://github.com/alcsel/Rust-Machine-Learning-Library-WIP" }
```

```rust
use rust_machine_learning::similarity_algorithms::levenstein_distance::levenshtein;

fn main() {
    println!("{}", levenshtein("kitten", "sitting")); // 3
}
```

## Examples

```bash
cargo run --example levenstein
```

## Testing

```bash
cargo test
```

## License
MIT