pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        Solution::coin_change_dp(coins, amount)
    }

    // this dp idea comes from the memoization implementation
    //
    // 5,4,3    7
    //
    //      1    2   3   4   5   6   7
    //  3   -1  -1   1   -1  -1  2   2
    //  4   -1  -1   -1  1   -1  -1  -1
    //  5   -1  -1   -1  -1  1   -1  -1
    //
    // 5,4,3    7
    //
    //      3   4   5
    //  0   0   0   0
    //  1   -1  -1  -1
    //  2   -1  -1  -1
    //  3   1   -1  -1
    //  4   -1  1   -1
    //  5   -1  -1  1
    //  6   2   -1  -1
    //  7   2   -1  -1
    //
    // 1,2,5    11
    //
    //      1   2   5
    //  1   1   -1  -1
    //  2   2   1   -1
    //  3   2   -1  -1
    //  4   3   2   -1
    //  5   3   -1  1
    //  6   2   3   -1
    //  7   3   2   -1
    //  8   3   4   -1
    //  9   4   3   -1
    //  10  4   5   2
    //  11  3   4   -1
    pub fn coin_change_dp(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount <= 0 {
            return 0;
        }
        coins.sort_unstable_by(|a, b| a.cmp(b));
        let mut dp = vec![vec![0i32; coins.len()]; (1 + amount) as usize];
        for n in 1..=amount as usize {
            for i in 0..coins.len() {
                let d = n as i32 - coins[i];
                if d < 0 {
                    dp[n][i] = -1;
                } else if d == 0 {
                    dp[n][i] = 1;
                } else {
                    dp[n][i] = 1 + *dp[d as usize][i..]
                        .iter()
                        .filter(|x| **x != -1)
                        .min()
                        .unwrap_or(&-2);
                }
            }
        }
        *dp[amount as usize][..]
            .iter()
            .filter(|x| **x != -1)
            .min()
            .unwrap_or(&-1)
    }

    // wrong implementation! However, you can get the idea for the
    // dynamic programming solution
    pub fn coin_change2(mut coins: Vec<i32>, amount: i32) -> i32 {
        use std::collections::HashMap;

        fn backtrack_memoization(
            coins: &[i32],
            index: usize,
            amount: i32,
            count: &mut i32,
            min_count: &mut i32,
            mem: &mut HashMap<(usize, i32), i32>,
        ) -> bool {
            println!(
                "coin={} amount={amount} count={} min_count={}",
                coins[index], *count, *min_count
            );
            if amount == 0 {
                return true;
            }
            // if let Some(n) = mem.get(&(index, amount)) {
            //     // println!("use memd coin={} amount={amount} n={n}", coins[index]);
            //     return false;
            //     // if *n == -1 {
            //     //     return false;
            //     // } else {
            //     //     *count += *n;
            //     //     return true;
            //     // }
            // }
            let mut possible = false;
            for i in index..coins.len() {
                let left = amount - coins[i];
                if left >= 0 {
                    possible = true;
                    *count += 1;
                    if backtrack_memoization(coins, i, left, count, min_count, mem) {
                        if *count < *min_count {
                            // println!("update min_count to {}", *count);
                            *min_count = *count;
                        }
                        // stop iterating current level
                        // *count -= 1;
                        // break;
                    }
                    *count -= 1;
                }
            }
            if !possible {
                mem.insert((index, amount), -1);
            }
            false
        }
        if amount == 0 {
            return 0;
        }
        let mut count = 0;
        let mut min_count = i32::MAX;
        // decrement
        coins.sort_unstable_by(|a, b| b.cmp(a));
        let mut mem = HashMap::new();
        backtrack_memoization(&coins, 0, amount, &mut count, &mut min_count, &mut mem);
        if min_count == i32::MAX {
            -1
        } else {
            min_count
        }
    }

    pub fn coin_change1(mut coins: Vec<i32>, amount: i32) -> i32 {
        // https://excalidraw.com/#json=V72d10JDhjU4ZG8ap_abM,kYVItAkUmeoRoPpROWkxEQ
        // test_multiple_solutions requires to find all paths leading to zero and use the minimum
        // count coins as the optimized path
        fn backtrack(
            coins: &[i32],
            index: usize,
            amount: i32,
            count: &mut i32,
            min_count: &mut i32,
        ) -> bool {
            // println!(
            //     "coin={} amount={amount} count={} min_count={}",
            //     coins[index], *count, *min_count
            // );
            if amount == 0 {
                return true;
            }
            for i in index..coins.len() {
                let left = amount - coins[i];
                if left >= 0 {
                    *count += 1;
                    if backtrack(coins, i, left, count, min_count) {
                        if *count < *min_count {
                            // println!("update min_count to {}", *count);
                            *min_count = *count;
                        }
                    }
                    *count -= 1;
                }
            }
            false
        }
        if amount == 0 {
            return 0;
        }
        let mut count = 0;
        let mut min_count = i32::MAX;
        // decrement
        coins.sort_unstable_by(|a, b| b.cmp(a));
        backtrack(&coins, 0, amount, &mut count, &mut min_count);
        if min_count == i32::MAX {
            -1
        } else {
            min_count
        }
    }
}
