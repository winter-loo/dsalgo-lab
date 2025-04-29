use alien_dictionary::Solution;

#[test]
fn test_example_1() {
    // Input: words = ["wrt","wrf","er","ett","rftt"]
    // Output: "wertf"
    let words = vec![
        "wrt".to_string(),
        "wrf".to_string(),
        "er".to_string(),
        "ett".to_string(),
        "rftt".to_string(),
    ];
    
    let result = Solution::alien_order(words);
    // Since there could be multiple valid orderings, we need to verify that the result is valid
    assert!(result == "wertf" || is_valid_ordering(&result, &["wrt", "wrf", "er", "ett", "rftt"]));
}

#[test]
fn test_example_2() {
    // Input: words = ["z","x"]
    // Output: "zx"
    let words = vec!["z".to_string(), "x".to_string()];
    
    assert_eq!(Solution::alien_order(words), "zx");
}

#[test]
fn test_example_3() {
    // Input: words = ["z","x","z"]
    // Output: ""
    let words = vec!["z".to_string(), "x".to_string(), "z".to_string()];
    
    assert_eq!(Solution::alien_order(words), "");
}

#[test]
fn test_single_word() {
    // Input: words = ["abc"]
    // Output: "abc" (or any permutation of "abc")
    let words = vec!["abc".to_string()];
    
    let result = Solution::alien_order(words);
    // With a single word, any permutation of the unique letters is valid
    assert_eq!(result.len(), 3);
    assert!(result.contains('a'));
    assert!(result.contains('b'));
    assert!(result.contains('c'));
}

#[test]
fn test_prefix_case() {
    // Input: words = ["ab", "adc"]
    // Output: "abcd" (or any valid ordering where 'b' comes before 'c')
    let words = vec!["ab".to_string(), "adc".to_string()];
    
    let result = Solution::alien_order(words);
    // Check that the result contains all unique letters
    assert_eq!(result.len(), 4);
    assert!(result.contains('a'));
    assert!(result.contains('b'));
    assert!(result.contains('c'));
    assert!(result.contains('d'));
    
    // Check that 'b' comes before 'c' in the result
    let b_pos = result.find('b').unwrap();
    let c_pos = result.find('c').unwrap();
    assert!(b_pos < c_pos);
}

// Helper function to check if the given ordering is valid for the words
fn is_valid_ordering(ordering: &str, words: &[&str]) -> bool {
    // Create a map from character to its position in the ordering
    let mut char_order = std::collections::HashMap::new();
    for (i, c) in ordering.chars().enumerate() {
        char_order.insert(c, i);
    }
    
    // Check if the words are in lexicographical order according to the given ordering
    for i in 0..words.len() - 1 {
        let word1 = words[i];
        let word2 = words[i + 1];
        
        let min_len = word1.len().min(word2.len());
        let mut found_diff = false;
        
        for j in 0..min_len {
            let c1 = word1.chars().nth(j).unwrap();
            let c2 = word2.chars().nth(j).unwrap();
            
            if c1 != c2 {
                found_diff = true;
                if char_order.get(&c1).unwrap() > char_order.get(&c2).unwrap() {
                    return false;
                }
                break;
            }
        }
        
        // If no difference was found and word1 is longer than word2, the ordering is invalid
        if !found_diff && word1.len() > word2.len() {
            return false;
        }
    }
    
    true
}
