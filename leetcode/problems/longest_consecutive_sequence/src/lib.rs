use std::collections::{BTreeSet, HashSet};

// Original implementation using BTreeSet
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut ns: BTreeSet<_> = nums.iter().collect();
    let mut ans = 1;
    let mut prev = ns.pop_first().unwrap();
    let mut run_size = 1;
    for n in ns {
        if *n == prev + 1 {
            run_size += 1;
        } else {
            ans = ans.max(run_size);
            run_size = 1;
        }
        prev = n;
    }
    ans.max(run_size)
}

// Functional implementation using HashSet
pub fn longest_consecutive_functional(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    // Convert to HashSet for O(1) lookups
    let num_set: HashSet<i32> = nums.into_iter().collect();
    
    // Find the start of each sequence and calculate its length
    num_set.iter()
        .filter(|&&num| !num_set.contains(&(num - 1))) // Start of a sequence
        .map(|&start| {
            // Calculate length of sequence starting at 'start'
            (1..).take_while(|i| num_set.contains(&(start + i)))
                .last()
                .map_or(1, |last| last + 1)
        })
        .max()
        .unwrap_or(0)
}