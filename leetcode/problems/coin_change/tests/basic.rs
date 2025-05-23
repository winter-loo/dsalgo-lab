use coin_change::Solution;

#[test]
fn test_example_1() {
    // Input: coins = [1,2,5], amount = 11
    // Output: 3
    // Explanation: 11 = 5 + 5 + 1
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(Solution::coin_change(coins, amount), 3);
}

#[test]
fn test_example_2() {
    // Input: coins = [2], amount = 3
    // Output: -1
    let coins = vec![2];
    let amount = 3;
    assert_eq!(Solution::coin_change(coins, amount), -1);
}

#[test]
fn test_example_3() {
    // Input: coins = [1], amount = 0
    // Output: 0
    let coins = vec![1];
    let amount = 0;
    assert_eq!(Solution::coin_change(coins, amount), 0);
}

#[test]
fn test_multiple_solutions() {
    // Multiple ways to make the amount, should return the minimum
    let coins = vec![1, 3, 4, 5];
    let amount = 7;
    assert_eq!(Solution::coin_change(coins, amount), 2); // 3 + 4 or 7 + 1 + 1
}

#[test]
fn test_large_amount() {
    // Larger amount that requires multiple coins
    let coins = vec![2, 5, 10, 25];
    let amount = 99;
    assert_eq!(Solution::coin_change(coins, amount), 7); // 25 + 25 + 25 + 10 + 10 + 2 + 2
}

#[test]
fn test_exact_coin() {
    // Amount exactly matches one of the coins
    let coins = vec![2, 5, 10];
    let amount = 10;
    assert_eq!(Solution::coin_change(coins, amount), 1);
}

#[test]
fn test_example_4() {
    let coins = vec![186, 419, 83, 408];
    let amount = 6249;
    assert_eq!(Solution::coin_change(coins, amount), 20);
}

#[test]
// #[ignore]
fn test_perf() {
    // Amount exactly matches one of the coins
    let coins = vec![411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422];
    let amount = 9864;
    assert_eq!(Solution::coin_change(coins, amount), 24);
}
