pub struct Solution;

impl Solution {
    // S=[1, 4, 2, 7, 6, 8]
    // buy at day i, sell at day j
    //
    // f(S,i)=S[j]-S[i] + f(S,j+1) for j > i
    //
    //              8   6   7   2   4   1
    //   B\S    6   5   4   3   2   1   0
    //        +--------------------------
    //      6 | 0   0   0   0   0   0   0
    //  8   5 | 0   0   0   0   0   0   0
    //  6   4 | 0   2   0   0   0   0   0
    //  7   3 | 0   1   -1  0   0   0   0
    //  2   2 | 0   6   4   5   0   0   0
    //  4   1 | 0   4   2   3   -2  0   0
    //  1   0 | 0   7   5   6   2   4   0
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp = vec![vec![0; prices.len() + 1]; prices.len() + 1];
        dp[prices.len() - 1][prices.len()] = prices[prices.len() - 1] - prices[prices.len() - 2];

        for i in (0..prices.len() - 2).rev() { // buy at day i
            for j in (0..prices.len()).rev() { // sell at day j
                if i < j {
                    println!("i={i} j={j}");
                    println!("dp={:#?}", dp);
                    // day (j+1) is the cooldown day
                    dp[i][j] = prices[j] - prices[i] + if j + 2 < dp[0].len() { *dp[j + 2][..].iter().max().unwrap() } else { 0 };
                }
            }
        }
        *dp[0][..].iter().max().unwrap() as i32
    }
}
