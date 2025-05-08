pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Solution::longest_palindrome_thinking_recursively(s)
    }

    // cbbd
    //
    // f(s, left, right):
    //  if left == right, then f(s, left + 1, right - 1)
    //  else f(s, left + 1, right) or f(s, left, right - 1)
    //
    // aacabdkacaa
    pub fn longest_palindrome_thinking_recursively(s: String) -> String {
        fn f(s: &[char], left: usize, right: usize, pleft: usize, pright: usize) -> (usize, usize) {
            // println!("s={s:?} left={left} right={right} pleft={pleft} pright={pright}");
            if left >= right {
                return (pleft, pright);
            }
            if s[left] == s[right] {
                f(s, left + 1, right - 1, pleft, pright)
            } else {
                let (l1, r1) = f(s, pleft, right - 1, pleft, right - 1);
                let (l2, r2) = f(s, left + 1, pright, left + 1, pright);
                if r1 - l1 > r2 - l2 {
                    (l1, r1)
                } else {
                    (l2, r2)
                }
            }
        }

        let s: Vec<_> = s.chars().collect();
        let (pl, pr) = f(&s, 0, s.len() - 1, 0, s.len() - 1);
        s[pl..=pr].iter().collect()
    }

    // cbbd
    //
    // c        b       b       d
    // cb       bb      bd
    // cbb      bbd
    // cbbd
    pub fn longest_palindrome_brute_force(s: String) -> String {
        let (mut left, mut right, mut mlen) = (0, 0, 0);
        for i in 0..s.len() {
            for j in i + 1..s.len() {
                if Solution::is_palindrom(&s, i, j) && j - i + 1 > mlen {
                    left = i;
                    right = j;
                    mlen = j - i + 1;
                }
            }
        }
        s.chars().skip(left).take(right - left + 1).collect()
    }

    pub fn is_palindrom(s: &str, i: usize, j: usize) -> bool {
        // println!("s={s} i={i} j={j}");
        let len = j - i + 1;
        let s1 = s.chars().skip(i).take(len / 2);
        let s2 = s
            .chars()
            .rev()
            .skip((s.len() - j).saturating_sub(1))
            .take(len / 2);
        s1.zip(s2).all(|(p, s)| p == s)
    }
}
