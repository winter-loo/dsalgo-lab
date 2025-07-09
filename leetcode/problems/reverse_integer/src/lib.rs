pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut sum = 0i32;
        while x != 0 {
            // sum = sum * 10 + x % 10;
            if let Some(s) = sum.checked_mul(10).and_then(|n| n.checked_add(x % 10)) {
                sum = s;
            } else {
                return 0;
            }
            x /= 10;
        }
        sum
    }
}
