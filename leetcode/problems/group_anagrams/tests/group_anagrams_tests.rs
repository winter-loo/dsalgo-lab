use group_anagrams::group_anagrams;

#[test]
fn test_group_anagrams_basic() {
    // Test case 1: Basic functionality
    let strs1 = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let mut result1 = group_anagrams(strs1);
    
    // Sort the results for consistent comparison
    for group in &mut result1 {
        group.sort();
    }
    result1.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut expected1 = vec![
        vec!["bat".to_string()],
        vec!["nat".to_string(), "tan".to_string()],
        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
    ];
    
    // Sort the expected results for consistent comparison
    for group in &mut expected1 {
        group.sort();
    }
    expected1.sort_by(|a, b| a[0].cmp(&b[0]));
    
    assert_eq!(result1, expected1);
}

#[test]
fn test_group_anagrams_empty_strings() {
    // Test case 2: Empty string
    let strs2 = vec!["".to_string()];
    let result2 = group_anagrams(strs2);
    let expected2 = vec![vec!["".to_string()]];
    assert_eq!(result2, expected2);
    
    // Test case: Multiple empty strings
    let strs_multi_empty = vec!["".to_string(), "".to_string(), "".to_string()];
    let mut result_multi_empty = group_anagrams(strs_multi_empty);
    
    // Ensure all empty strings are grouped together
    for group in &mut result_multi_empty {
        group.sort();
    }
    result_multi_empty.sort_by(|a, b| a.len().cmp(&b.len()));
    
    let expected_multi_empty = vec![vec!["".to_string(), "".to_string(), "".to_string()]];
    assert_eq!(result_multi_empty, expected_multi_empty);
}

#[test]
fn test_group_anagrams_single_char() {
    // Test case 3: Single character
    let strs3 = vec!["a".to_string()];
    let result3 = group_anagrams(strs3);
    let expected3 = vec![vec!["a".to_string()]];
    assert_eq!(result3, expected3);
    
    // Test with all single letters
    let strs_all_letters = vec![
        "a".to_string(), "b".to_string(), "c".to_string(), 
        "a".to_string(), "b".to_string(), "c".to_string()
    ];
    let mut result_all_letters = group_anagrams(strs_all_letters);
    
    // Normalize for comparison
    for group in &mut result_all_letters {
        group.sort();
    }
    result_all_letters.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let expected_all_letters = vec![
        vec!["a".to_string(), "a".to_string()],
        vec!["b".to_string(), "b".to_string()],
        vec!["c".to_string(), "c".to_string()],
    ];
    assert_eq!(result_all_letters, expected_all_letters);
}