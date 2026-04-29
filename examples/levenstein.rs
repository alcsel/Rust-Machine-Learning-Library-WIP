// examples/levenstein.rs
use rust_machine_learning::similarity_algorithms::levenstein_distance::levenshtein;
fn main() {
    println!("1. kelime:");
    let a = read_line();
    println!("2. kelime:");
    let b = read_line();
    println!("Mesafe: {}", levenshtein(&a, &b));
}

fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}