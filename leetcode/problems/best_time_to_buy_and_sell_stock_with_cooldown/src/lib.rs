pub struct Solution;

impl Solution {
    // buy at day i, sell at day j
    //
    // for j > i, if S[j] > S[i]:
    //
    //  f(S,i) = Max(S[j]-S[i] + f(S,j+2), f(S,i+1))
    //
    // if S[j] <= S[i]:
    //
    //  f(S,i) = f(S,i+1)
    //
    // [6, 1, 6, 4, 3, 0, 2]
    //
    //
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp = vec![0; prices.len() + 1];

        for i in (0..prices.len() - 1).rev() {
            // buy at day i
            for j in (i + 1..prices.len()).rev() {
                // sell at day j
                // println!("i={i} j={j}");
                // println!("dp={:?}", dp);
                let profit = prices[j] - prices[i];
                if profit > 0 {
                    // day (j+1) is the cooldown day
                    dp[i] = [dp[i], dp[i + 1], profit + dp.get(j + 2).unwrap_or(&0)]
                        .into_iter()
                        .max()
                        .unwrap();
                } else {
                    dp[i] = dp[i].max(dp[i + 1]);
                }
            }
        }
        dp[0] as i32
    }
}
