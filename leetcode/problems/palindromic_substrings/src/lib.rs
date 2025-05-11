pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0;
        let s: Vec<_> = s.chars().collect();
        for i in 0..s.len() {
            ans += expand_outwards(&s, i, i);
            ans += expand_outwards(&s, i, i + 1);
        }

        ans as i32
    }
}

fn expand_outwards(s: &[char], left: usize, right: usize) -> usize {
    let mut l = left as isize;
    let mut r = right;
    let mut count = 0;
    while l >= 0 && r < s.len() && s[l as usize] == s[r] {
        l -= 1;
        r += 1;
        count += 1;
    }
    count
}
