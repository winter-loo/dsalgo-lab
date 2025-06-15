pub struct Solution;

impl Solution {
    // Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
    // f(s3, s1, s2) = true or false?
    // g(s3, s1, s2, i3, i1, i2) = g(s3, s1, s2, i3 - i, i1 - j, i2 - k) if j + k == i and
    //                              s3[i3..i3+j] = s1[i1..i1+j]
    //                              s3[i3+j..i3+j+k] = s2[i2..i2+k]
    // f(s3, s1, s2) = g(s3, s1, s2, 0, 0, 0)
    //
    // As i3 = i1 + i2, so,
    //
    // g(s3, s1, s2, i1, i2) = g(s3, s1, s2, i1 - j, i2 - k) and
    //                              s3[i1+i2..i1+i2+j] = s1[i1..i1+j]
    //                              s3[i1+i2+j..i1+i2+j+k] = s2[i2..i2+k]
    //
    //
    // h(s3, s1, s2) = h("dbbcbcac", "bcc", "dbbca") as prefix "aa" of s3 found in s1
    // and prefix "aad" of s3 not found in s1 or s2.
    //
    // h("dbbcbcac", "bcc", "dbbca") = h("bbcbcac", "bcc", "bbca") ||
    //                                 h("bcbcac", "bcc", "bca") ||
    //                                 h("cbcac", "bcc", "ca") ||
    //                                 h("bcac", "bcc", "a")
    // h("bcac", "bcc", "a") = h("ac", "c", "a")
    // h("ac", "c", "a") = h("c", "c", "")
    // h("c", "c", "") = h("", "", "") = true
    //
    // Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
    //
    //              a   a   b   c   c
    //   s2\s1  0   1   2   3   4   5
    //      0   T   T   T   F   F   F
    //  d   1   F   F   T   T   F   F
    //  b   2   F   F   T   T   T   F
    //  b   3   F   F   T
    //  c   4   F
    //  a   5   F
    //
    //  For g(s3, s1, s2, 2, 3), i.e, t1=aa,t2=dbb and t3=aadbb, as the last
    //  char of t3, 'b', is the last char of t2, so we should check the sub-problem
    //  g(s3,s1,s2,2,2), i.e., t1=aa, t2=db, t3=aadb
    //
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let mut dp = vec![vec![false; s1.len() + 1]; s2.len() + 1];

        for i in 0..=s2.len() {
            for j in 0..=s1.len() {
                if i == 0 && j == 0 {
                    dp[i][j] = true;
                    continue;
                }
                if j > 0 && s3[i + j - 1] == s1[j - 1] {
                    dp[i][j] |= dp[i][j - 1];
                }
                if i > 0 && !dp[i][j] && s3[i + j - 1] == s2[i - 1] {
                    dp[i][j] |= dp[i - 1][j];
                }
            }
        }

        dp[s2.len()][s1.len()]
    }
}
