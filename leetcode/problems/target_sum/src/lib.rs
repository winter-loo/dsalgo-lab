pub struct Solution;

impl Solution {
    // 1,1,1,1,1
    //
    // Here's thinking: '+' and '-' is the equivalent to 'select' and 'not select'.
    // So, '+' means select current element; '-' means not select current element.
    //
    // S=1,1,1,1,1 T=3
    // f(t,i) = f(t-S[i],i+1) + f(t+S[i],i+1) for i in [0,len(S)] and target is 't'.
    // The answer is `f(T,0)` which means staring from the index 0 of S, the number
    // of ways to sum to T.
    //
    // f(3, 0) = f(2, 1) + f(4, 1) = 4 + 1 = 5
    // f(2, 1) = f(1, 2) + f(3, 2) = 3 + 1 = 4
    // f(4, 1) = f(3, 2) + f(5, 2) = 1 + 0 = 1
    // f(1, 2) = f(0, 3) + f(2, 3) = 2 + 1 = 3
    // f(3, 2) = f(2, 3) + f(4, 3) = 1 + 0 = 1
    // f(5, 2) = f(4, 3) + f(6, 3) = 0 + 0 = 0
    // f(0, 3) = f(-1, 4) + f(1, 4) = 1 + 1 = 2
    // f(2, 3) = f(1, 4) + f(3, 4) = 1 + 0 = 1
    // f(4, 3) = f(3, 4) + f(5, 4) = 0 + 0 = 0
    // f(6, 3) = f(5, 4) + f(7, 4) = 0 + 0 = 0
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // problem constraint: `0 <= sum(nums[i]) <= 1000`
        // let n = nums.iter().sum::<i32>() + 1;
        // let mut dp = vec![vec![0; n as usize + 1]; nums.len()];
        let n = 1000;
        let mut dp = vec![vec![0; n as usize + 1]; nums.len()];
        // for v in &dp {
        //     println!("{:?}", v);
        // }
        // println!("--------------------");

        dp[nums.len() - 1][nums[nums.len() - 1] as usize] = nums
            .last()
            .is_some_and(|x| *x == 0)
            .then_some(2)
            .unwrap_or(1);
        for i in (0..nums.len() - 1).rev() {
            for j in 0..=n {
                let used = dp[i + 1].get((j - nums[i]).abs() as usize).unwrap_or(&0);
                let unused = dp[i + 1].get((j + nums[i]) as usize).unwrap_or(&0);
                dp[i][j as usize] = used + unused;
            }
            // for v in &dp {
            //     println!("{:?}", v);
            // }
            // println!("--------------------");
        }
        dp[0][target.abs() as usize]
    }
}
