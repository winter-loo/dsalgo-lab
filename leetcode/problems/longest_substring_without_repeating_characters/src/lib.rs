pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }

        let s = s.into_bytes();

        let mut ml = 0;
        let (mut l, mut r) = (0, 0);
        while r < s.len() - 1 {
            if let Some(i) = s[l..=r].iter().position(|b| *b == s[r + 1]) {
                l += i + 1;
            }
            r += 1;
            ml = ml.max(r - l + 1);
        }

        ml as i32
    }
}
