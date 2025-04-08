pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }
        for (i, c) in s.chars().enumerate() {
            println!("{i}:{c}");
        }

        let s = s.into_bytes();
        let mut ans = 0;

        let (mut l, mut r, mut changes) = (0, 1, 0);
        while r < s.len() {
            if s[r] != s[l] {
                changes += 1;
            }
            println!("before: l={l} r={r} changes={changes}");
            if changes > k {
                while s[l] == s[l + 1] {
                    l += 1;
                }
                l += 1;
                r = l + 1;
                changes = 0;
            } else {
                r += 1;
            }
            println!("after: l={l} r={r} changes={changes}");
            ans = ans.max(r - l);
        }

        let (mut l, mut r, mut changes) = (s.len() - 2, s.len() - 1, 0);
        loop {
            assert!(l < r && changes <= k);
            if s[l] != s[r] {
                changes += 1;
            }
            if changes > k {
                ans = ans.max(r - l);
                println!("after2: l={l} r={r} changes={changes}");

                while s[r] == s[r - 1] {
                    r -= 1;
                }
                r -= 1;
                l = r - 1;
                changes = 0;
            } else {
                l -= 1;
            }
            if l == 0 {
                ans = ans.max(r - l + 1);
                break;
            }
        }

        ans as i32
    }
}
