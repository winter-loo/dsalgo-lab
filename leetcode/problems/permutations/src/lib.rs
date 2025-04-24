pub struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        let mut results = vec![];
        for i in 0..nums.len() {
            nums.swap(0, i);
            let mut subperm = Solution::permute(nums[1..].to_vec());
            for v in subperm.iter_mut() {
                v.push(nums[0]);
            }
            results.extend(subperm.into_iter());
        }
        results
    }
}
