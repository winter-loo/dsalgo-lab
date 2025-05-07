pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        let (mut f_i1, mut f_i2) = (0, 0);
        let mut ans1 = 0;
        // do not choose the first and second slot
        for i in (2..nums.len()).rev() {
            ans1 = f_i1.max(nums[i] + f_i2);
            f_i2 = f_i1;
            f_i1 = ans1;
        }

        // choose the first slot
        // [1, 2, 3, 4, 5]
        let mut ans2 = 0;
        let (mut f_i1, mut f_i2) = (0, 0);
        for i in (2..nums.len() - 1).rev() {
            ans2 = f_i1.max(nums[i] + f_i2);
            f_i2 = f_i1;
            f_i1 = ans2;
        }
        ans2 += nums[0];

        let mut ans3 = 0;
        let (mut f_i1, mut f_i2) = (0, 0);
        for i in (3..nums.len()).rev() {
            ans3 = f_i1.max(nums[i] + f_i2);
            f_i2 = f_i1;
            f_i1 = ans3;
        }
        ans3 += nums[1];

        ans1.max(ans2).max(ans3)
    }

    pub fn rob_with_decision_tree_and_memoization(nums: Vec<i32>) -> i32 {
        fn traverse(nums: &[i32], start: usize, index: usize, mem: &mut [i32]) -> i32 {
            if start == 0 && index + 1 >= nums.len() || start == 1 && index >= nums.len() {
                return 0;
            }
            if mem[index] == i32::MAX {
                mem[index] = std::cmp::max(
                    traverse(nums, start, index + 2, mem) + nums[index],
                    traverse(nums, start, index + 1, mem),
                );
            }
            mem[index]
        }

        if nums.len() <= 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        let mut mem = vec![i32::MAX; nums.len()];
        let m1 = traverse(&nums, 0, 0, &mut mem);
        let mut mem = vec![i32::MAX; nums.len()];
        let m2 = traverse(&nums, 1, 1, &mut mem);
        m1.max(m2)
    }

    /// see [diagram](https://excalidraw.com/#json=RusYWmk9cIqLxn8ICqh_H,OHpdwoBQiyfzdNpChH3k4Q)
    pub fn rob_with_decision_tree_refactored(nums: Vec<i32>) -> i32 {
        fn traverse(nums: &[i32], start: usize, index: usize) -> i32 {
            if start == 0 && index + 1 >= nums.len() || start == 1 && index >= nums.len() {
                return 0;
            }
            std::cmp::max(
                traverse(nums, start, index + 2) + nums[index],
                traverse(nums, start, index + 1),
            )
        }

        if nums.len() <= 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        traverse(&nums, 0, 0).max(traverse(&nums, 1, 1))
    }

    /// see [diagram](https://excalidraw.com/#json=phOhe1lcop62t2hK2DYIC,sW9T1vLOjAAjqatpuLmE2w)
    pub fn rob_with_decision_tree_initial(nums: Vec<i32>) -> i32 {
        fn traverse(nums: &[i32], start: usize, index: usize, path_sum: &mut i32, mv: &mut i32) {
            if start == 0 && index + 1 >= nums.len() || start == 1 && index >= nums.len() {
                if *path_sum > *mv {
                    *mv = *path_sum;
                }
                return;
            }

            // select current index and go to the next choice `index + 2`
            *path_sum += nums[index];
            traverse(nums, start, index + 2, path_sum, mv);
            // throw current index and go to the next choice `index + 1`
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
