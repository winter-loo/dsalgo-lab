pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn dfs(index: usize, nums: &[i32], path: &mut Vec<i32>, results: &mut HashSet<Vec<i32>>) {
            {
                let mut p = path.to_vec();
                p.sort_unstable();
                results.insert(p);
            }

            for i in index..nums.len() {
                path.push(nums[i]);
                dfs(i + 1, nums, path, results);
                path.pop();
            }
        }
        let mut path = vec![];
        let mut results = HashSet::new();
        dfs(0, &nums[..], &mut path, &mut results);
        results.into_iter().collect()
    }
}
