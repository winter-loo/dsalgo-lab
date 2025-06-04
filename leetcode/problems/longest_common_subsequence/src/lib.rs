pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Solution::longest_common_subsequence_dp(text1, text2)
    }

    // Known S=abcde T=aceb f(S,T)=?
    //
    // g(S,T,I,J)=max(g(S,T,I+1,J+1)+1, g(S,T,I+1,J)) if S[I] == T[J]
    // or
    // g(S,T,I,J)=max(g(S,T,I,J-1), g(S, T, I+1, J)) if S[I] not in T[J..]
    //
    // S=abcde T=aceb
    //
    // See test_example_4
    //
    //              g   p   a   r   m   b   u
    //   I/J    7   6   5   4   3   2   1   0
    //       +-------------------------------
    //     6 |  0   0   0   0   0   0   0   0
    //  r  5 |  0   0   0   0   1   1   1   1
    //  k  4 |  0   0   0   0   1   1   1   1
    //  p  3 |  0   0   1   1   1   1   1   1
    //  u  2 |  0   0   1   1   1   1   1   2
    //  z  1 |  0   0   1   1   1   1   1   1
    //  e  0 |  0   0   1   1   1   1   1   1
    //
    pub fn longest_common_subsequence_dp(text1: String, text2: String) -> i32 {
        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();
        let mut dp = vec![vec![0; t2.len() + 1]; t1.len() + 1];

        let mut max_len = 0;
        for i in (0..t1.len()).rev() {
            for j in (0..t2.len()).rev() {
                if t1[i] == t2[j] {
                    dp[i][j] = (1 + dp[i + 1][j + 1]).max(dp[i + 1][j])
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j+1]);
                }
                max_len = max_len.max(dp[i][j]);
            }
        }
        max_len as i32
    }

    pub fn longest_common_subsequence_recur(text1: String, text2: String) -> i32 {
        fn recur(t1: &[char], i1: usize, t2: &[char], i2: usize) -> i32 {
            println!("i1={i1} i2={i2}");
            if i1 >= t1.len() {
                return 0;
            }
            let mut max_len = 0;
            let mut found = false;
            for j in i2..t2.len() { // to find matching char
                if t1[i1] == t2[j] {
                    found = true;
                    let m1 = 1 + recur(t1, i1 + 1, t2, j + 1);
                    let m2 = recur(t1, i1 + 1, t2, i2);
                    let m = m1.max(m2);
                    max_len = max_len.max(m);
                    break;
                }
            }
            if !found {
                max_len = max_len.max(recur(t1, i1 + 1, t2, i2));
            }
            max_len as i32
        }
        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();
        recur(&t1, 0, &t2, 0)
    }
}
