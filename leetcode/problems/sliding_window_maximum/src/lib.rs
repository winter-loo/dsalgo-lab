pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() || k == 0 {
            return Vec::new();
        }

        let k = k as usize;
        let n = nums.len();

        // For edge case where k=1, return the array itself
        if k == 1 {
            return nums;
        }

        let mut result = Vec::with_capacity(n - k + 1);

        // Deque to store indices of elements in decreasing order
        let mut deque: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            // Remove elements outside the current window
            while !deque.is_empty() && i + 1 > k && deque.front().unwrap() < &(i + 1 - k) {
                deque.pop_front();
            }

            // Remove smaller elements from the back as they can't be maximum
            while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
                deque.pop_back();
            }

            // Add current element's index
            deque.push_back(i);

            // Add the maximum (front of deque) to result if we have a full window
            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}
