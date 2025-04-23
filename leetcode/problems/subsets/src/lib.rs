pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut results = vec![];
        backtrack(0, &nums[..], &mut current, &mut results);
        results
    }
}

fn backtrack(index: usize, nums: &[i32], current: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
    results.push(current.to_vec());

    for i in index..nums.len() {
        current.push(nums[i]);
        backtrack(i + 1, nums, current, results);
        current.pop();
    }
}
