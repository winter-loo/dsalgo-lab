pub struct Solution;

impl Solution {
    // ((*)(*))(
    pub fn check_valid_string(s: String) -> bool {
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

        let missing = lp_count - rp_count;
        if missing == 0 {
            true
        } else {
            let mut mlp = 0;
            for c in s1 {
                if c == '(' {
                    mlp += 1;
                } else if c == '*' {
                    mlp -= 1;
                }
            }
            if mlp > 0 {
                false
            }
            star_count >= missing
        }
    }
}
