pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..=nums.len()).sum::<usize>() as i32 - nums.iter().sum::<i32>()
    }
}
