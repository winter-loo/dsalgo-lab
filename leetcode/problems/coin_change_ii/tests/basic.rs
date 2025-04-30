use coin_change_ii::Solution;

#[test]
fn test_example_1() {
    // Input: amount = 5, coins = [1,2,5]
    // Output: 4
    // Explanation: there are four ways to make up the amount:
    // 5=5
    // 5=2+2+1
    // 5=2+1+1+1
    // 5=1+1+1+1+1
    let amount = 5;
    let coins = vec![1, 2, 5];
    assert_eq!(Solution::change(amount, coins), 4);
}

#[test]
fn test_example_2() {
    // Input: amount = 3, coins = [2]
    // Output: 0
    // Explanation: the amount of 3 cannot be made up just with coins of 2.
    let amount = 3;
    let coins = vec![2];
    assert_eq!(Solution::change(amount, coins), 0);
}

#[test]
fn test_example_3() {
    // Input: amount = 10, coins = [10]
    // Output: 1
    let amount = 10;
    let coins = vec![10];
    assert_eq!(Solution::change(amount, coins), 1);
}

#[test]
fn test_zero_amount() {
    // Amount 0 should always have 1 way (by not using any coin)
    let amount = 0;
    let coins = vec![1, 2, 5];
    assert_eq!(Solution::change(amount, coins), 1);
}

#[test]
fn test_multiple_coins() {
    // Test with multiple coins
    let amount = 10;
    let coins = vec![1, 2, 5];
    assert_eq!(Solution::change(amount, coins), 10);
}

#[test]
fn test_large_amount() {
    // Test with a larger amount
    let amount = 500;
    let coins = vec![3, 5, 7, 8, 9, 10, 11];
    assert_eq!(Solution::change(amount, coins), 35502874);
}
