pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Ok(n) = nums.binary_search(&target) {
            n as i32
        } else {
            -1
        }
    }
}
