pub struct Solution;

impl Solution {
    // Input: s = "ab", p = ".*"
    //
    // s[i] p[j]
    //
    // if p[j] == '.' || p[j] == s[i], matches!
    // if p[j] == '*', if p[j-1] == '.' || p[j-1] == s[i] matches!
    //
    // f(s,p,i,j) = ?
    //
    // assuming s[0..i] matches p[0..j], compare s[i+1] with p[j+1].
    //
    //            .   *
    //  i\j   0   1   2
    //    0   T   F   T
    //  a 1   F   T   T
    //  b 2   F   F   T
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;

        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();

        for i in 0..dp.len() {
            for j in 1..dp[0].len() {
                if i == 0 {
                    if p[j - 1] == '*' {
                        dp[i][j] = dp[i][j - 2];
                    }
                    continue;
                }
                if s[i - 1] == p[j - 1] || p[j - 1] == '.' {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[j - 1] == '*' {
                    if p[j - 2] == '.' || p[j - 2] == s[i - 1] {
                        dp[i][j] = dp[i - 1][j] || dp[i][j - 2];
                    } else {
                        dp[i][j] = dp[i][j - 2];
                    }
                }
            }
        }
        dp[dp.len() - 1][dp[0].len() - 1]
    }
}
