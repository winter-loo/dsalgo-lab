pub struct Solution;

impl Solution {
    // [1,5,11,5]
    // [1]      |    [5,11,5]
    // [5]      |    [1,11,5]
    // [11]     |    [1,5,5]
    // [5]      |    [1,5,11]
    //
    // [1] [5]      |       [11,5]
    // [1] [11]     |       [5,5]
    // [1] [5]      |       [11,5]
    //
    // [1] [5] [11]     |       [5]
    // [1] [5] [5]      |       [11]
    //
    // is there existing `i` such that sum(0..i) == sum(i..n) ?
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        fn recur(nums: &mut [i32], left_sum: i32) -> bool {
            if nums.iter().sum::<i32>() == left_sum {
                return true;
            }
            if nums.len() <= 1 {
                return false;
            }
            for i in 0..nums.len() {
                nums.swap(i, 0);
                let n = nums[0];
                if recur(&mut nums[1..], left_sum + n) {
                    return true;
                }
            }
            false
        }
        recur(&mut nums, 0)
    }
}
