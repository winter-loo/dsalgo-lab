#![allow(unused)]

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // sort and two pointers
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = -nums[i];
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                if nums[l] + nums[r] > target {
                    r -= 1;
                } else if nums[l] + nums[r] < target {
                    l += 1;
                } else {
                    ans.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < nums.len() && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while r > l && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }
        ans
    }

    pub fn three_sum_sort_and_binary_search_optimized(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();
        // println!("nums={nums:?}");

        let mut ans = vec![];
        // [-4, -2, -3, -1, -1, 0, 0, 1, 2, 3, 4]
        for l in 0..nums.len() - 2 {
            // remove duplicates
            if l != 0 && nums[l] == nums[l - 1] {
                continue;
            }
            for r in (l + 2..nums.len()).rev() {
                // remove duplicates
                if r != nums.len() - 1 && nums[r] == nums[r + 1] {
                    continue;
                }
                let target = -nums[l] - nums[r];
                // println!("l={l} r={r} {} {}", nums[l], nums[r]);
                match nums[l + 1..r].binary_search(&target) {
                    Ok(m) => {
                        // println!("find");
                        ans.push(vec![nums[l], nums[l + 1 + m], nums[r]]);
                    }
                    Err(m) => {
                        // [-4, -1, -1, 0, 1, 9]
                        // -4 + 9
                        // [-4, -1, -1, 0, 1, 1]
                        // -4 + 1
                        // println!("not found, {}", m);
                        if l + 1 + m <= l || l + 1 + m >= r - 1 {
                            break;
                        }
                    }
                }
            }
        }
        ans
    }

    pub fn three_sum_sort_and_binary_search(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = vec![];

        for l in 0..nums.len() - 2 {
            // remove duplicates
            if l != 0 && nums[l] == nums[l - 1] {
                continue;
            }
            for r in (l + 2..nums.len()).rev() {
                // remove duplicates
                if r != nums.len() - 1 && nums[r] == nums[r + 1] {
                    continue;
                }
                let target = -nums[l] - nums[r];
                if let Ok(m) = nums[l + 1..r].binary_search(&target) {
                    ans.push(vec![nums[l], nums[l + 1 + m], nums[r]]);
                }
            }
        }
        ans
    }

    pub fn three_sum_brute_force(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut ans = HashSet::new();
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut thr = [nums[i], nums[j], nums[k]];
                        thr.sort_unstable();
                        ans.insert(thr);
                    }
                }
            }
        }
        ans.into_iter().map(|a| a.to_vec()).collect()
    }
}
