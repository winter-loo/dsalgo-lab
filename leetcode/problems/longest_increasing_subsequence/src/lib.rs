pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Solution::length_of_lis_starting_at_i(nums)
    }

    pub fn length_of_lis_starting_at_i(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            for j in i..nums.len() {
                if nums[j] > nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap() as i32
    }

    pub fn length_of_lis_ending_at_i(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap() as i32
    }
}
