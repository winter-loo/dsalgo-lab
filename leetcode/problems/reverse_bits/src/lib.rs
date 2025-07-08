pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut a = 0;
        for i in (0..32).rev() {
            a += ((x >> (32 - i - 1)) & 0x01) << i;
        }
        a
    }
}
