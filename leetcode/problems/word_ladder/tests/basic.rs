use word_ladder::Solution;

#[test]
fn test_example_1() {
    // Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
    // Output: 5
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];
    
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
}

#[test]
fn test_example_2() {
    // Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
    // Output: 0
    // Explanation: The endWord "cog" is not in wordList, so there is no valid transformation sequence.
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
    ];
    
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
}

#[test]
fn test_no_transformation() {
    // No transformation sequence exists
    let begin_word = "hit".to_string();
    let end_word = "xyz".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];
    
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
}

#[test]
fn test_single_transformation() {
    // Only one transformation needed
    let begin_word = "hit".to_string();
    let end_word = "hot".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
    ];
    
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 2);
}

#[test]
fn test_multiple_paths() {
    // Multiple paths exist, should return the shortest
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
        "dit".to_string(),
        "dig".to_string(),
    ];
    
    // Shortest path: hit -> hot -> dot -> dog -> cog (5 words)
    // Alternative path: hit -> dit -> dig -> dog -> cog (5 words)
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
}
