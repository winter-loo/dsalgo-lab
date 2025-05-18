pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        Solution::change_dp_1d(amount, coins)
    }

    // 1,2,5    5
    //
    //      1   2   5
    // 0    0   0   0
    // 1    1   0   0
    // 2    1   1   0
    // 3    2   0   0
    // 4    2   1   0
    // 5    3   0   1
    //
    pub fn change_dp_2d(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        }
        let mut dp = vec![vec![0; coins.len()]; (1 + amount) as usize];
        for n in 1..=amount as usize {
            for i in 0..coins.len() {
                let d = n as i32 - coins[i];
                if d < 0 {
                    dp[n][i] = 0;
                } else if d == 0 {
                    dp[n][i] = 1;
                } else {
                    dp[n][i] = dp[d as usize][i..].iter().sum();
                }
            }
        }
        dp[amount as usize][..].iter().sum()
    }

    // 1,2,5    5
    //
    //      1   2   5
    // 0    1   1   1 => 1
    // 1    1   0   0 => 1
    // 2    1   1   0 => 2
    // 3    2   0   0 => 2
    // 4    2   1   0 => 3
    // 5    3   0   1 => 4
    //
    pub fn change_dp_1d(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; (amount + 1) as usize];
        dp[0] = 1;
        for i in 0..coins.len() {
            for j in 1..=amount as usize {
                if j >= coins[i] as usize {
                    dp[j] += dp[j - coins[i] as usize];
                }
            }
        }
        dp[amount as usize]
    }
}
