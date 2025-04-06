pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 2 {
            return vec![];
        }
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            let sum = numbers[l] + numbers[r];
            if sum > target {
                r -= 1;
            } else if sum < target {
                l += 1;
            } else {
                return vec![1 + l as i32, 1 + r as i32]; // exactly one solution
            }
        }
        vec![]
    }
}
