pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut unique_candidates = HashMap::new();
        for n in candidates {
            if n == 0 {
                panic!("zero is not allowed in the candidates")
            }
            *unique_candidates.entry(n).or_insert(0) += 1;
        }

        fn backtrack(index: usize, candidates: &[i32], counts: &[usize], target: i32, path: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
            // println!("path: {path:?}");
            if target < 0 {
                return;
            }
            if target == 0 {
                results.push(path.to_vec());
                return;
            }
            for i in index..candidates.len() {
                let mut count = counts[i].min((target / candidates[i]) as usize).max(1);
                path.extend(std::iter::repeat_n(candidates[i], count));
                loop {
                    backtrack(i + 1, candidates, counts, target - (count as i32 * candidates[i]), path, results);
                    path.pop();
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
            }
        }

        let mut path = vec![];
        let mut results = vec![];
        let (candidates, counts): (Vec<i32>, Vec<usize>) = (unique_candidates.keys().copied().collect(), unique_candidates.values().copied().collect());
        // println!("candidates: {candidates:?}");
        // println!("counts: {counts:?}");
        backtrack(0, &candidates[..], &counts[..], target, &mut path, &mut results);
        results.into_iter().collect()
    }

    // XXX: Time Limit Exceeded
    // how to handle efficiently "[1, 1, 1, 1, 1], 2"?
    // * one possible optimization is to use map
    // 1 -> 5
    // [2, 5, 2, 1, 2], 5
    // 1 -> 1, 2 -> 3, 5 -> 1
    // conclusion: this is a solution to try to list unique candidates
    pub fn combination_sum2_iterate_all(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn backtrack(index: usize, candidates: &[i32], target: i32, path: &mut Vec<i32>, results: &mut HashSet<Vec<i32>>) {
            println!("index={index:?}");
            if target < 0 {
                return;
            }
            if target == 0 {
                let mut path = path.to_vec();
                path.sort_unstable();
                results.insert(path);
                return;
            }
            for i in index..candidates.len() {
                path.push(candidates[i]);
                backtrack(i + 1, candidates, target - candidates[i], path, results);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut results = HashSet::new();
        backtrack(0, &candidates[..], target, &mut path, &mut results);
        results.into_iter().collect()
    }
}
