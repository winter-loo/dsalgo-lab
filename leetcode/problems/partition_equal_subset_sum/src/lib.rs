pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        Solution::can_partition_dp_1d(nums)
    }

    // from the 2d solution, we could know our dp only depends on the previous
    // row, so it's a waste to keep other rows in memory.
    pub fn can_partition_dp_1d(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum as usize / 2;
        let mut prev = vec![false; target + 1];
        let mut curr = vec![false; target + 1];
        prev[0] = true;
        curr[0] = true;

        for i in 1..=nums.len() {
            for j in 1..=target {
                if nums[i - 1] > j as i32 {
                    curr[j] = prev[j];
                } else {
                    curr[j] = prev[j] || prev[(j as i32 - nums[i - 1]) as usize];
                }
            }
            prev = curr.clone();
        }
        prev[target]
    }

    pub fn can_partition_dp_2d(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum as usize / 2;
        // question is transformed into: Are there numbers being added to `target`
        // It is a classical problem in computer science: subset sum problem
        // tabulation formula:
        // f(A, i, S) = [
        //      f(A, i - 1, S) or f(A, i - 1, S - A[i]), if A[i] <= S,
        //      f(A, i - 1, S), if A[i] > S, 
        // ]
        // [2, 2, 1, 1], T=3
        //          0   1   2   3   S
        //  0       T   F   F   F
        //  1[2]    T   F   T   F
        //  2[2]    T   F   T   F
        //  3[1]    T   T   T   T 
        //  4[1]    T   T   T   T
        //  A[i]
        let mut dp = vec![vec![false; target + 1]; nums.len() + 1];

        // if sum is 0, the answer is true(empty set)
        for i in 0..=nums.len() {
            dp[i][0] = true;
        }

        for i in 1..=nums.len() {
            for j in 1..=target {
                if nums[i- 1] > j as i32 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][(j as i32 - nums[i - 1]) as usize];
                }
            }
        }
        dp[nums.len()][target]
    }

    // https://excalidraw.com/#json=H3l21ZnRM1xoij-K4Uv3t,RgQoanhiL5H8Y0Qy6-sbbA
    pub fn can_partition_with_memoization(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        fn recur(nums: &[i32], sum: i32, target: i32, mem: &mut HashSet<(usize, i32)>) -> bool {
            if sum == target {
                return true;
            }
            if nums.is_empty() || sum > target || mem.get(&(nums.len(), sum)).is_some() {
                return false;
            }
            // not take it
            if recur(&nums[1..], sum, target, mem) {
                return true;
            }
            // take it
            if recur(&nums[1..], sum + nums[0], target, mem) {
                return true;
            }
            mem.insert((nums.len(), sum));
            false
        }
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;

        let mut mem = HashSet::new();
        recur(&nums, 0, target, &mut mem)
    }

    // [1,5,11,5]
    // [1]      |    [5,11,5]
    // [5]      |    [1,11,5]
    // [11]     |    [1,5,5]
    // [5]      |    [1,5,11]
    //
    // [1] [5]      |       [11,5]
    // [1] [11]     |       [5,5]
    // [1] [5]      |       [11,5]
    //
    // [1] [5] [11]     |       [5]
    // [1] [5] [5]      |       [11]
    //
    // is there existing `i` such that sum(0..i) == sum(i..n) ?
    //
    // TODO: how to add memoization to this recursive function
    pub fn can_partition_recur(mut nums: Vec<i32>) -> bool {
        fn recur(nums: &mut [i32], left_sum: i32, right_sum: i32) -> bool {
            if right_sum == left_sum {
                return true;
            }
            if nums.len() <= 1 || left_sum > right_sum {
                return false;
            }
            for i in 0..nums.len() {
                nums.swap(i, 0);
                let n = nums[0];
                if recur(&mut nums[1..], left_sum + n, right_sum - n) {
                    return true;
                }
            }
            false
        }
        let right_sum = nums.iter().sum();
        recur(&mut nums, 0, right_sum)
    }
}
