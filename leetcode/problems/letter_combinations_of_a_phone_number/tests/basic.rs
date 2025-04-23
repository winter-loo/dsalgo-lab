use letter_combinations_of_a_phone_number::Solution;
use std::collections::HashSet;

#[test]
fn test_example_1() {
    // Input: digits = "23"
    // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    let digits = "23".to_string();
    let result = Solution::letter_combinations(digits);
    
    // Convert result to HashSet for easy comparison regardless of order
    let result_set: HashSet<String> = result.into_iter().collect();
    
    // Expected combinations
    let expected = vec![
        "ad".to_string(), "ae".to_string(), "af".to_string(),
        "bd".to_string(), "be".to_string(), "bf".to_string(),
        "cd".to_string(), "ce".to_string(), "cf".to_string()
    ];
    let expected_set: HashSet<String> = expected.into_iter().collect();
    
    assert_eq!(result_set, expected_set);
}

#[test]
fn test_example_2() {
    // Input: digits = ""
    // Output: []
    let digits = "".to_string();
    let result = Solution::letter_combinations(digits);
    
    assert!(result.is_empty());
}

#[test]
fn test_example_3() {
    // Input: digits = "2"
    // Output: ["a","b","c"]
    let digits = "2".to_string();
    let result = Solution::letter_combinations(digits);
    
    // Convert result to HashSet for easy comparison regardless of order
    let result_set: HashSet<String> = result.into_iter().collect();
    
    // Expected combinations
    let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let expected_set: HashSet<String> = expected.into_iter().collect();
    
    assert_eq!(result_set, expected_set);
}

#[test]
fn test_multiple_digits() {
    // Input: digits = "234"
    // Output: 3*3*3 = 27 combinations
    let digits = "234".to_string();
    let result = Solution::letter_combinations(digits);
    
    // There should be 3*3*3 = 27 combinations
    assert_eq!(result.len(), 27);
    
    // Check a few expected combinations
    let result_set: HashSet<String> = result.into_iter().collect();
    assert!(result_set.contains("adg"));
    assert!(result_set.contains("bfi"));
    assert!(result_set.contains("cei"));
}

#[test]
fn test_digits_with_four_letters() {
    // Input: digits = "7"
    // Output: ["p","q","r","s"]
    let digits = "7".to_string();
    let result = Solution::letter_combinations(digits);
    
    // Convert result to HashSet for easy comparison regardless of order
    let result_set: HashSet<String> = result.into_iter().collect();
    
    // Expected combinations
    let expected = vec![
        "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()
    ];
    let expected_set: HashSet<String> = expected.into_iter().collect();
    
    assert_eq!(result_set, expected_set);
}
