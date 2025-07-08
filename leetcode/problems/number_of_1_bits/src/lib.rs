pub struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut count = 0;
        while n != 0 {
            if n & 0x01 == 1 {
                count += 1;
            }
            n >>= 1;
        }
        count
    }
}
