use best_time_to_buy_and_sell_stock_with_cooldown::Solution;

#[test]
fn test_example_1() {
    // Input: prices = [1,2,3,0,2]
    // Output: 3
    // Explanation: transactions = [buy, sell, cooldown, buy, sell]
    let prices = vec![1, 2, 3, 0, 2];
    assert_eq!(Solution::max_profit(prices), 3);
}

#[test]
fn test_example_2() {
    // Input: prices = [1]
    // Output: 0
    let prices = vec![1];
    assert_eq!(Solution::max_profit(prices), 0);
}

#[test]
fn test_decreasing_prices() {
    // Decreasing prices should result in 0 profit
    let prices = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::max_profit(prices), 0);
}

#[test]
fn test_multiple_transactions() {
    // Test with multiple possible transactions
    let prices = vec![1, 2, 3, 0, 2, 5, 6, 4, 8];
    assert_eq!(Solution::max_profit(prices), 10);
}

#[test]
fn test_cooldown_impact() {
    // Test where cooldown affects the optimal strategy
    let prices = vec![1, 4, 2, 7, 6, 8];
    assert_eq!(Solution::max_profit(prices), 7);
    // [1, 4, 2, 7, 6, 8]
    // [0, 0, 0, 0, 0, 0]
    // [0, 0, 6, 2, 2, 0]
    // [7, 6, 6, 2, 2, 0]
}

#[test]
fn test_example_3() {
    let prices = vec![2, 1, 4];
    assert_eq!(Solution::max_profit(prices), 3);
}

#[test]
fn test_example_4() {
    let prices = vec![6, 1, 6, 4, 3, 0, 2];
    assert_eq!(Solution::max_profit(prices), 7);
    // [6, 1, 6, 4, 3, 0, 2]
    //
    //
    //              2   0   3   4   6   1   6
    //          7   6   5   4   3   2   1   0
    //      +--------------------------------
    //    7 |   0   0
    //  2 6 |   0   0
    //  0 5 |   0   2   0
    //  3 4 |   0   2   2   0
    //  4 3 |   0   2   2   2   0
    //  6 2 |   0   2   2   2   2   0
    //  1 1 |   0   2   2   2   5   7   0
    //  6 0 |   0   7   7   7   7   7   7   0
}
