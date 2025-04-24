pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if tasks.is_empty() {
            return 0;
        }
        if n == 0 {
            return tasks.len() as i32;
        }

        // 1. Count frequencies
        let mut counts = [0; 26];
        for &task in &tasks {
            counts[(task as u8 - b'A') as usize] += 1;
        }

        // 2. Find max frequency
        let max_freq = *counts.iter().max().unwrap_or(&0);
        if max_freq == 0 { // Should not happen if tasks is not empty, but safe guard
            return 0;
        }

        // 3. Count tasks with max frequency
        let max_count = counts.iter().filter(|&&count| count == max_freq).count() as i32;

        // 4. Calculate ideal time based on max frequency frame
        // Frame: (max_freq - 1) intervals of (n + 1) slots each, plus the final max_count tasks
        let frame_length = (max_freq - 1) * (n + 1) + max_count;

        // 5. Return the max of calculated time and total tasks
        // The total time must be at least the number of tasks.
        max(frame_length, tasks.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::least_interval(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::least_interval(vec!['A','A','A','B','B','B'], 0), 6);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::least_interval(vec!['A','A','A','A','A','A','B','C','D','E','F','G'], 2), 16);
    }

     #[test]
    fn test_single_task_type() {
        assert_eq!(Solution::least_interval(vec!['A','A','A'], 2), 7); // A _ _ A _ _ A
    }

    #[test]
    fn test_all_different_tasks() {
        assert_eq!(Solution::least_interval(vec!['A','B','C'], 1), 3);
    }

     #[test]
    fn test_more_tasks_than_frame() {
         // max_freq = 2 (A), max_count = 1
         // frame = (2-1)*(3+1) + 1 = 4 + 1 = 5
         // tasks.len() = 6
        assert_eq!(Solution::least_interval(vec!['A','A','B','C','D','E'], 3), 6);
    }
}
