pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        println!("nums={nums:?}");
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = (l + r) / 2;
            println!("m={m} l={l} r={r}");
            if nums[m] < nums[l] {
                if nums[m] < nums[m - 1] {
                    return nums[m];
                }
                r = m - 1;
            }

            if nums[m] > nums[r] {
                if nums[m] > nums[m + 1] {
                    return nums[m + 1];
                }
                l = m + 1;
            }

            if nums[m] == nums[l] {
                return nums[m];
            }
        }
        unreachable!()
    }
}
