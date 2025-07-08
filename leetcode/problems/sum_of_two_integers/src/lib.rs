pub struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut carry = 0;
        while b != 0 {
            carry = a & b;
            a = a ^ b;
            b = carry << 1;
        }
        a
    }
}
