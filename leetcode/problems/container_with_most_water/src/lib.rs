pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut ma = 0;
        while l < r {
            println!("l={l} r={r} {} {}", height[l], height[r]);
            let area = height[l].min(height[r]) * (r - l) as i32;
            if area > ma {
                ma = area;
            }
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ma
    }
}
