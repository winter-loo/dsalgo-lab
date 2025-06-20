pub struct Solution;

impl Solution {
    // Input: word1 = "horse", word2 = "ros"
    // operations: i, r, d
    // if length of word1 is longer than word2's, then no 'i' operations needed!
    // if the same length, only 'r' operations can be applied!
    //
    //
    // Compare (h)ouse with (r)os
    // h != r → if 'r', then we get 'rouse'; if 'd', then we get 'ouse'
    //
    // house ┌─r─> rouse
    //       └─d─> ouse
    //
    // Compare r(o)use with (r)os, the same, no chane
    // Compare ro(u)se with ro(s)
    // u != s → if 'r', then we get 'rosse'; if 'd', then we get 'rose'
    //
    // ro(u)se ┌─r─> rosse
    //         └─d─> rose
    //
    // Now we done with W2 but not done with W1, apply 'd' operations.
    //
    // ro(u)se ┌─r─> rosse ──d──> ros
    //         └─d─> rose ──d──> ros
    //
    // ────────────────────────────────────────────────────────────────────
    //
    // Now consider 'ouse' with 'ros'
    //
    // Compare (o)use with (r)os
    //
    // (o)use ┌─r─> r(u)se ┌─r─> rose ─d─> ros
    //        │            └─d─> r(s)e ─r─> ro(e) ─r─> ros
    //        └─d─> (u)se ─r─> r(s)e ─r─> ro(e) ─r─> ros
    //
    // ──────────────────────────────────────────────────────────────────────
    //
    // Summary:
    //
    // (h)ouse ┌─r─> r(o)use ─> ro(u)se ┌─r─> rosse ─d─> ros 
    //         │                        └─d─> rose ─d─> ros
    //         └─d─> (o)use ┌─r─> r(u)se ┌─r─> rose ─d─> ros
    //                      │            └─d─> r(s)e ─r─> ro(e) ─r─> ros
    //                      └─d─> (u)se ─r─> r(s)e ─r─> ro(e) ─r─> ros
    //
    // Done!
    //
    // ───────────────────────Tabulation────────────────────────────────────
    //
    // W1 = "house", W2 = "ros", so we only need 'r' and 'd' operations.
    // f(W1,W2,i,j)=?
    //
    //              h   o   u   s   e
    //              1   2   3   4   5
    //          0   1   2   3   4   5
    //        r 1   1   ?
    //        o 2
    //        s 3
    //
    // for F=f(W1,W2,2,1),
    // - if we choose 'd' operation, then F=f(W1,W2,1,1)+1=2
    // - if we choose 'r' operation, then F=f(W1,W2,1,0)+1=2
    // So, the minimum operations we need is 2.
    //
    //                  h   o   u   s   e
    //              0   1   2   3   4   5
    //            ┌───────────────────────
    //          0 │ 0   1   2   3   4   5
    //        r 1 │ 1   1   2   3   4   5
    //        o 2 │ 2   ?
    //        s 3 │ 3
    //
    // when W1 is empty, we need 'i' operation. The number of 'i' operation is
    // the length of the W2. Hence the above first column.
    //
    // when W2 is empty, we need 'd' operation. The number of 'd' operator is
    // the length of the W1. Hence the above first row.
    //
    // for G=g(1,2),
    // - if we choose 'r' operation, then G=g(0,1)+1=2
    // - if we choose 'd' operation, then G=g(0,2)+1=3
    // NOTE: if W1[i] = W2[j], no need to add '1' to previous result.
    //
    // ────────────────────BUG IN ABOVE SOLUTION─────────────────────────────
    //
    //                  a
    //              0   1
    //          ┌─────────
    //        0 │   0   1
    //      a 1 │   1   0
    //      b 2 │   2   ?
    // 
    // consider g(1,2)=?
    // - if choose 'r', then g(1,2)=1+g(0,0)=2
    // - if choose 'd', then g(1,2)=1+g(0,2)=3
    // We need still consider 'i' operation,
    // - if choose 'i', then g(1,2)=1+g(1,1)=1 
    //
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<_> = word1.chars().collect();
        let word2: Vec<_> = word2.chars().collect();
        let mut dp = vec![vec![0; word1.len() + 1]; word2.len() + 1];

        //
        // set initial values
        //
        // first row
        for i in 0..=word1.len() {
            dp[0][i] = i;
        }
        // first column
        for (i, row) in dp.iter_mut().enumerate() {
            row[0] = i;
        }

        // println!("initial dp=");
        // print!("{:<8}", " ");
        // for c in word1.iter() {
        //     print!("{c:<4}");
        // }
        // println!();
        // for (i, row) in dp.iter().enumerate() {
        //     print!("{:<4}", if i == 0 { ' ' } else { word2[i - 1] });
        //     for v in row.iter() {
        //         print!("{:<4}", v);
        //     }
        //     println!();
        // }

        // dynamic programming
        for i in 1..=word2.len() {
            for j in 1..=word1.len() {
                // use 'r' operation, then dp[i-1][j-1]
                // use 'd' operation, then dp[i][j-1]
                // if they are equal, no operation needed
                // use 'i' operation, then dp[i-1][j]
                let r_cost = if word1[j - 1] == word2[i - 1] { 0 } else { 1 };
                dp[i][j] = std::cmp::min(dp[i-1][j-1] + r_cost, dp[i][j-1] + 1);
                dp[i][j] = dp[i][j].min(1 + dp[i-1][j]);
                // println!("────────────────────");
                // print!("{:<8}", " ");
                // for c in word1.iter() {
                //     print!("{c:<4}");
                // }
                // println!();
                // for (k, row) in dp.iter().enumerate() {
                //     print!("{:<4}", if k == 0 { ' ' } else { word2[k - 1] });
                //     for v in row.iter() {
                //         print!("{:<4}", v);
                //     }
                //     println!();
                // }
            }
        }
        dp[word2.len()][word1.len()] as i32
    }
}
