use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    let mut max_freq = 0;
    let mut freq = HashMap::new();
    for n in nums.iter() {
        *freq.entry(*n).or_insert(1) += 1;
        max_freq = max_freq.max(*freq.get(n).unwrap());
    }

    let mut buckets = vec![vec![]; max_freq + 1];
    for (n, count) in freq {
        buckets[count as usize].push(n);
    }

    let mut ans = vec![];
    for ns in buckets.iter().rev() {
        ans.extend(ns);
        if ans.len() >= k as usize {
            ans.truncate(k as usize);
            return ans;
        }
    }
    ans
}
