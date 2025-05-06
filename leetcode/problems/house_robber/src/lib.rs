pub struct Solution;

impl Solution {
    // intuition: current maximum value depends on future values
    pub fn rob(nums: Vec<i32>) -> i32 {
        // f(i) = max(nums[i] + f(i+2), f(i+1))
        let (mut f_i1, mut f_i2) = (0, 0);
        let mut ans = 0;
        for i in (0..nums.len()).rev() {
            ans = f_i1.max(nums[i] + f_i2);
            f_i2 = f_i1;
            f_i1 = ans;
        }
        ans
    }
}
