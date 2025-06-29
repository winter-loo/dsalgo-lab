use hand_of_straights::Solution;

#[test]
fn test_example_1() {
    // Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
    // Output: true
    // Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]
    let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let group_size = 3;
    assert!(Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_example_2() {
    // Input: hand = [1,2,3,4,5], groupSize = 4
    // Output: false
    // Explanation: Alice's hand can't be rearranged into groups of 4.
    let hand = vec![1, 2, 3, 4, 5];
    let group_size = 4;
    assert!(!Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_single_card_groups() {
    // Test with group size of 1
    let hand = vec![5, 1, 3, 7];
    let group_size = 1;
    assert!(Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_impossible_division() {
    // Test when hand size is not divisible by group size
    let hand = vec![1, 2, 3, 4, 5];
    let group_size = 2;
    assert!(!Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_missing_cards() {
    // Test when there are missing cards in the sequence
    let hand = vec![1, 2, 4, 5, 6, 7];
    let group_size = 3;
    assert!(!Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_duplicate_cards() {
    // Test with duplicate cards
    let hand = vec![1, 1, 2, 2, 3, 3];
    let group_size = 3;
    assert!(Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_multiple_valid_groups() {
    // Test with multiple valid groups
    let hand = vec![1, 2, 3, 4, 5, 6];
    let group_size = 3;
    assert!(Solution::is_n_straight_hand(hand, group_size));
}

#[test]
fn test_large_numbers() {
    // Test with large card values
    let hand = vec![
        1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005,
    ];
    let group_size = 2;
    assert!(Solution::is_n_straight_hand(hand, group_size));
}
