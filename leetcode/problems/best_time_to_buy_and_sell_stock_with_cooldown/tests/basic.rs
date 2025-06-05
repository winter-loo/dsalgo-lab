use best_time_to_buy_and_sell_stock_with_cooldown::Solution;

#[test]
fn test_example_1() {
    // Input: prices = [1,2,3,0,2]
    // Output: 3
    // Explanation: transactions = [buy, sell, cooldown, buy, sell]
    let prices = vec![1, 2, 3, 0, 2];
    assert_eq!(Solution::max_profit(prices), 3);
    // [1, 2, 3, 0, 2]
    //
    //              2   0   3   2   1
    //          5   4   3   2   1   0
    //        +-----------------------
    //     5  | 0   0   0   0   0   0
    //  2  4  | 0   0   0   0   0   0
    //  0  3  | 0   2   0   0   0   0
    //  3  2  | 0   -1  -2  0   0   0
    //  2  1  | 0   0   -2  1   0   0
    //  1  0  | 0   1   -1  2   3   0
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
    // S=[1, 4, 2, 7, 6, 8]
    // buy at day i, sell at day j
    //
    // f(S,i)=S[j]-S[i] + f(S,j+1) for j > i
    //
    //          8   6   7   2   4   1
    //   B\S    5   4   3   2   1   0
    //        +--------------------------
    //  8   5 | 0   0   0   0   0   0
    //  6   4 | 2   0   0   0   0   0
    //  7   3 | 1   -1  0   0   0   0
    //  2   2 | 6   4   5   0   0   0
    //  4   1 | 4   2   3   -2  0   0
    //  1   0 | 7   5   6   2   4   0
}
