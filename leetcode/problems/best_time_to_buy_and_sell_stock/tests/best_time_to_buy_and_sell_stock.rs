use best_time_to_buy_and_sell_stock::Solution;

#[test]
fn test_example_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(Solution::max_profit(prices), 5);
}

#[test]
fn test_example_2() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(Solution::max_profit(prices), 0);
}

#[test]
fn test_single_price() {
    let prices = vec![1];
    assert_eq!(Solution::max_profit(prices), 0);
}

#[test]
fn test_two_prices_increasing() {
    let prices = vec![1, 2];
    assert_eq!(Solution::max_profit(prices), 1);
}

#[test]
fn test_two_prices_decreasing() {
    let prices = vec![2, 1];
    assert_eq!(Solution::max_profit(prices), 0);
}

#[test]
fn test_fluctuating_prices() {
    let prices = vec![3, 2, 6, 5, 0, 3];
    assert_eq!(Solution::max_profit(prices), 4);
}
