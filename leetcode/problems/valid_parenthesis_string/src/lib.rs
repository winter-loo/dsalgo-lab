pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        Solution::check_valid_string_with_key_observation(s)
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
        // println!("lp_count={lp_count} rp_count={rp_count} star_count={star_count}");

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
            // println!("found_star={found_star} m={m}");
            if !found_star && m > 0 {
                false
            } else {
                star_count >= missing
            }
        }
    }

    // ((*)(*))(
    // systematic way but Time Limit Exceeded
    pub fn check_valid_string_system(s: String) -> bool {
        let mut s: Vec<_> = s.chars().collect();

        fn inner(s: &mut [char], i: usize, stack: &mut Vec<char>) -> bool {
            // println!("s={s:?} i={i} stack={stack:?}");
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
                } else if s[j] == ')' {
                    // stack is empty and the ')' occurring
                    // println!("impossible");
                    return false;
                } else {
                    stack.push(s[j]);
                }
                // println!("stack={stack:?}");
                j += 1;
            }
            if j == s.len() {
                stack.is_empty()
            } else {
                let mut stack2 = stack.clone();
                if inner(s, j + 1, &mut stack2) {
                    return true;
                }
                let mut stack2 = stack.clone();
                s[j] = '(';
                if inner(s, j, &mut stack2) {
                    return true;
                }
                let mut stack2 = stack.clone();
                s[j] = ')';
                if inner(s, j, &mut stack2) {
                    return true;
                }
                // backtracking
                s[j] = '*';
                false
            }
        }

        let mut stack = vec![];
        inner(&mut s, 0, &mut stack)
    }

    // ((*)(*))(
    // First observation: no asterisk after the last '(', so invalid.
    // So the question we can ask is: how many asterisks do we have after each '('?
    //
    // **()))
    // Second observation: for '()))', it could be reduced to '))'. As we have
    // two asterisks before, so valid.
    // So the question we can ask is: how many asterisks do we have before ')'?
    //
    // ((*)(*))(
    //
    //
    //      0    1   2   3   4   5   6   7   8
    //      (    (   *   )   (   *   )   )   (
    //
    // how many asterisks do we have after each '('?
    //
    //      0    1   2   3   4   5   6   7   8
    //      (    (   *   )   (   *   )   )   (
    //      2    2           1               0
    //
    // how many asterisks do we have before ')'?
    //
    //      0    1   2   3   4   5   6   7   8
    //      (    (   *   )   (   *   )   )   (
    //                   1           2   2
    //
    // how many pairs could we remove?
    //
    //      0    1   2   3   4   5   6   7   8
    //      x    x   *   x   x   *   x   x   (
    //
    // At this point we could know how many asterisks after the last '(', which
    // is 0. So invalid.
    //
    pub fn check_valid_string_with_key_observation(s: String) -> bool {
        let mut asterisks_after_lp = vec![0; s.len()];
        let mut asterisks_before_rp = vec![0; s.len()];

        let mut count = 0;
        // scan from the tail
        for (i, c) in s.chars().rev().enumerate() {
            if c == '*' {
                count += 1;
            } else if c == '(' {
                asterisks_after_lp[s.len() - i - 1] = count;
            }
        }

        count = 0;
        // scan from the head
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                count += 1;
            } else if c == ')' {
                asterisks_before_rp[i] = count;
            }
        }
        // println!("asterisks_after_lp={asterisks_after_lp:?}");
        // println!("asterisks_before_rp={asterisks_before_rp:?}");

        let mut stack = vec![];
        let mut count_lp = 0;
        let mut count_rp = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                count_lp += 1;
                stack.push((c, i));
            } else if c == ')' {
                if stack.last().is_some_and(|d| d.0 == '(') {
                    count_lp -= 1;
                    stack.pop();
                } else if stack.is_empty() && asterisks_before_rp[i] == 0 {
                    return false;
                } else {
                    count_rp += 1;
                    stack.push((c, i));
                }
            }
        }
        // println!("final stack={stack:?} count_lp={count_lp} count_rp={count_rp}");

        // At final, the stack could have:
        // ((
        // ))
        // )))((
        // It is impossible to have:
        // ()

        // check state in the stack
        if stack.is_empty() {
            true
        } else {
            // check every element in the stack
            let mut counting_lp = 0;
            let mut counting_rp = 0;
            for (c, i) in stack {
                if c == '(' {
                    counting_lp += 1;
                    if count_lp - counting_lp + 1 > asterisks_after_lp[i] {
                        return false;
                    }
                } else if c == ')' {
                    counting_rp += 1;
                    if counting_rp > asterisks_before_rp[i] {
                        return false;
                    }
                }
            }
            true
        }
    }
}
