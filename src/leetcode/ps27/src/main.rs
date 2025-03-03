/// [27. Remove Element](https://leetcode.com/problems/remove-element/)
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
      let (mut i, mut j) = (0, 0);
      while j < nums.len() {
          if nums[j] != val {
              (nums[i], nums[j]) = (nums[j], nums[i]);
              i += 1;
              j += 1;
          } else {
              j += 1;
          }
      }
      i as i32 
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let result = Solution::remove_element(&mut nums, val);
    println!("Result: {}", result);
    println!("Array: {:?}", nums);
}
