pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut results = vec![];

        fn backtrack(
            index: usize,
            candidates: &[i32],
            target: i32,
            path: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            if target < 0 {
                return;
            }
            if target == 0 {
                results.push(path.to_vec());
                return;
            }
            for i in index..candidates.len() {
                path.push(candidates[i]);
                backtrack(i, candidates, target - candidates[i], path, results);
                path.pop();
            }
        }

        backtrack(0, &candidates[..], target, &mut path, &mut results);
        results
    }

    pub fn combination_sum_initial(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut results = HashSet::new();
        backtrack_with_set(&candidates[..], target, &mut path, &mut results);
        results.into_iter().collect()
    }
}

fn backtrack_with_set(
    candidates: &[i32],
    target: i32,
    path: &mut Vec<i32>,
    results: &mut HashSet<Vec<i32>>,
) {
    if target < 0 {
        return;
    }
    if target == 0 {
        let mut path = path.to_vec();
        path.sort_unstable();
        results.insert(path);
        return;
    }
    for n in candidates {
        path.push(*n);
        backtrack_with_set(candidates, target - n, path, results);
        path.pop();
    }
}
