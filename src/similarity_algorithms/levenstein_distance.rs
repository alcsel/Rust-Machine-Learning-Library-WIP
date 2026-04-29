/// Calculates the Levenshtein distance (edit distance) between two strings.
/// 
/// The Levenshtein distance is a string metric for measuring the difference between two sequences.
/// Informally, the Levenshtein distance between two words is the minimum number of single-character 
/// edits (insertions, deletions or substitutions) required to change one word into the other.
///
/// # Arguments
/// * `a` - The first string to compare.
/// * `b` - The second string to compare.
///
/// # Returns
/// The minimum edit distance between the two strings (as a `usize`).
pub fn levenshtein(a: &str, b: &str) -> usize {
    // Convert strings to vectors of characters (Vec<char>) to properly handle 
    // Unicode characters (e.g. multi-byte characters, emojis).
    // If we only looked at bytes, multi-byte characters would cause errors.
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    
    // Character lengths of both strings
    let m = a.len();
    let n = b.len();
    
    // Create a DP (Dynamic Programming) matrix.
    // dp[i][j] holds the distance between the first i characters of 'a' and the first j characters of 'b'.
    // We set the matrix size to (m+1) x (n+1) and initially fill each cell with 0.
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // Base cases:
    // To transform the first i characters of the first string into an empty string,
    // we need i "deletion" operations.
    for i in 0..=m { dp[i][0] = i; }
    
    // To transform an empty string into the first j characters of the second string,
    // we need j "insertion" operations.
    for j in 0..=n { dp[0][j] = j; }

    // Start filling the matrix step by step.
    for i in 1..=m {
        for j in 1..=n {
            // If the characters currently being compared are the same, no extra cost is needed (cost = 0).
            dp[i][j] = if a[i-1] == b[j-1] {
                dp[i-1][j-1]
            } else {
                // If characters are different; 
                // We pick the minimum cost among substitution, deletion, or insertion,
                // and add 1 to it for the current operation cost.
                1 + dp[i-1][j-1]     // Substitution
                    .min(dp[i-1][j]) // Deletion
                    .min(dp[i][j-1]) // Insertion
            };
        }
    }

    // The value at the bottom right corner of the matrix is the computed distance between the full strings.
    dp[m][n]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kitten_sitting() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }

    #[test]
    fn test_rust_dust() {
        assert_eq!(levenshtein("rust", "dust"), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
    }
}