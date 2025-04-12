pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // println!("s={s}, t={t}");
        if s.len() < t.len() || s.is_empty() || t.is_empty() {
            return String::from("");
        }

        let mut t_counts = HashMap::new();
        for c in t.chars() {
            *t_counts.entry(c).or_insert(0) += 1;
        }

        let s: Vec<char> = s.chars().collect();
        let mut win_counts = HashMap::new();
        for i in 0..t.len() {
            *win_counts.entry(s[i]).or_insert(0) += 1;
        }

        let mut found = false;
        let (mut lm, mut rm) = (0, s.len() - 1);
        let (mut l, mut r) = (0, t.len() - 1);
        while r < s.len() {
            // println!("l={l} r={r}");
            // println!("win_counts={win_counts:?}");
            let mut includes = true;
            for (chr, n) in t_counts.iter() {
                if !win_counts.get(chr).is_some_and(|m| *m >= *n) {
                    includes = false;
                    break;
                }
            }
            if includes {
                if !found {
                    found = true;
                }
                // minimum successful len
                if r - l + 1 == t.len() {
                    return s[l..=r].iter().collect();
                }
                if r - l <= rm - lm {
                    (lm, rm) = (l, r);
                }
                if let Some(n) = win_counts.get_mut(&s[l]) {
                    *n -= 1;
                    if *n == 0 {
                        win_counts.remove(&s[l]);
                    }
                }
                l += 1;
                while t_counts.get(&s[l]).is_none() {
                    win_counts.remove(&s[l]);
                    l += 1;
                }
            } else {
                r += 1;
                if r >= s.len() {
                    break;
                }
                *win_counts.entry(s[r]).or_insert(0) += 1;
            }
        }

        if found {
            s[lm..=rm].iter().collect()
        } else {
            String::from("")
        }
    }
}
