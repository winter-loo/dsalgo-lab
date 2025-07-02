pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        Solution::check_valid_string_system(s)
    }

    // ((*)(*))(
    // buggy mind!!!
    pub fn check_valid_string_buggy(s: String) -> bool {
        use std::collections::HashMap;

        let s1: Vec<_> = s.chars().collect();

        let freq = s
            .chars()
            .map(|c| (c, 1))
            .fold(HashMap::new(), |mut acc, kv| {
                *acc.entry(kv.0).or_insert(0) += 1;
                acc
            });

        let rp_count = *freq.get(&')').unwrap_or(&0);
        let lp_count = *freq.get(&'(').unwrap_or(&0);
        let star_count = *freq.get(&'*').unwrap_or(&0);
        println!("lp_count={lp_count} rp_count={rp_count} star_count={star_count}");

        let missing = lp_count - rp_count;
        if missing == 0 {
            true
        } else {
            let mut found_star = true;
            let mut m = 0;
            for c in s1 {
                if c == '(' {
                    m += 1;
                    found_star = false;
                } else if c == '*' && !found_star {
                    found_star = true;
                } else if c == ')' {
                    m -= 1;
                }
            }
            println!("found_star={found_star} m={m}");
            if !found_star && m > 0 {
                false
            } else {
                star_count >= missing
            }
        }
    }

    // ((*)(*))(
    // systematic way
    pub fn check_valid_string_system(s: String) -> bool {
        let mut s: Vec<_> = s.chars().collect();

        fn inner(s: &mut [char], i: usize, stack: &mut Vec<char>) -> bool {
            if i == s.len() {
                return stack.is_empty();
            }
            let mut j = i;
            while j < s.len() {
                if s[j] == '*' {
                    break;
                }
                // process until next '*'
                if let Some(c) = stack.last() {
                    if *c == '(' && s[j] == ')' {
                        stack.pop();
                    } else {
                        stack.push(s[j]);
                    }
                }
                j += 1;
            }
            if j == s.len() {
                stack.is_empty()
            } else {
                inner(s, j + 1, stack)
                    || {
                        s[j] = '(';
                        inner(s, j - 1, stack)
                    }
                    || {
                        s[j] = ')';
                        inner(s, j - 1, stack)
                    }
            }
        }

        let mut stack = vec![];
        inner(&mut s, 0, &mut stack)
    }
}
