pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        Solution::rob_with_decision_tree_refactored(nums)
    }

    pub fn rob_with_decision_tree_refactored(nums: Vec<i32>) -> i32 {
        fn traverse(nums: &[i32], start: usize, index: usize) -> i32 {
            if start == 0 && index + 1 >= nums.len() || start == 1 && index >= nums.len() {
                return 0;
            }
            std::cmp::max(
                traverse(nums, start, index + 1) + nums[index],
                traverse(nums, start, index + 1),
            )
        }

        traverse(&nums, 0, 0).max(traverse(&nums, 1, 1))
    }

    // see https://excalidraw.com/#json=phOhe1lcop62t2hK2DYIC,sW9T1vLOjAAjqatpuLmE2w
    pub fn rob_with_decision_tree_initial(nums: Vec<i32>) -> i32 {
        fn traverse(nums: &[i32], start: usize, index: usize, path_sum: &mut i32, mv: &mut i32) {
            if start == 0 && index + 1 >= nums.len() || start == 1 && index >= nums.len() {
                if *path_sum > *mv {
                    *mv = *path_sum;
                }
                return;
            }

            *path_sum += nums[index];
            traverse(nums, start, index + 2, path_sum, mv);
            *path_sum -= nums[index];
            traverse(nums, start, index + 1, path_sum, mv);
        }

        if nums.len() <= 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        let mut mv1 = 0;
        let mut mv2 = 0;
        let mut path_sum = 0;
        traverse(&nums, 0, 0, &mut path_sum, &mut mv1);
        traverse(&nums, 1, 1, &mut path_sum, &mut mv2);
        mv1.max(mv2)
    }
}
