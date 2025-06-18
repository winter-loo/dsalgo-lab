pub struct Solution;

impl Solution {
    // Input: s = "rabbbit", t = "rabbit"
    // f(s,t,i,j)=f(s,t,i+1,j+1) + f(s,t,i+1,j) if s[i] == t[j]
    // or f(s,t,i+1,j) if s[i] != t[j]
    //
    //     t\s          t   i   b   b   b   a   r
    //              7   6   5   4   3   2   1   0
    //            ┌──────────────────────────────
    //         6  │ 1   1   1   1   1   1   1   1
    //     t   5  │ 0   1   1   1   1   1   1   1
    //     i   4  │ 0   0   1   1   1   1   1   1
    //     b   3  │ 0   0   0   1   2   3   3   3
    //     b   2  │ 0   0   0   0   1   3   3   3
    //     a   1  │ 0   0   0   0   0   0   3   3
    //     r   0  │ 0   0   0   0   0   0   0   3
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
        // initial value
        for i in 0..=s.len() {
            dp[t.len()][i] = 1;
        }

        for i in (0..t.len()).rev() {
            for j in (0..s.len()).rev() {
                // println!("i={i} j={j}");
                // for i in (0..=t.len()).rev() {
                //     for j in (0..=s.len()).rev() {
                //         print!("{}", dp[i][j]);
                //     }
                //     println!();
                // }
                if s[j] == t[i] {
                    dp[i][j] = dp[i + 1][j + 1] + dp[i][j + 1];
                } else {
                    dp[i][j] = dp[i][j + 1];
                }
            }
        }
        dp[0][0] as i32
    }
}
