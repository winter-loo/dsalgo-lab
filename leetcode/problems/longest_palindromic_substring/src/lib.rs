pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Solution::longest_palindrome_outwards(s)
    }

    // only works for odd number of characters
    pub fn longest_palindrome_outwards(s: String) -> String {
        let s: Vec<_> = s.chars().collect();        
        let (mut mlen, mut ml, mut mr) = (0, 0, 0);
        if s.len() % 2 == 1 {
            for i in 1..s.len() {
                let mut l = i - 1;
                let mut r = i + 1;
                while r < s.len() {
                    if s[l] != s[r] {
                        break;
                    }
                    let (l2, overflowed) = l.overflowing_sub(1);
                    if overflowed {
                        break;
                    }
                    l = l2;
                    r += 1;
                }
                if r - l + 1 > mlen {
                    mlen = r - l + 1;
                    ml = l;
                    mr = r;
                }
            }
        }
        s[ml..=mr].into_iter().collect()
    }

    // cbbd
    //
    // f(s, left, right):
    //  if left == right, then f(s, left + 1, right - 1)
    //  else f(s, left + 1, right) or f(s, left, right - 1)
    //
    // The above formula is wrong. Anti example: "xaabacxcabaaxcabaax"
    pub fn longest_palindrome_thinking_recursively(s: String) -> String {
        fn f(s: &[char], left: usize, right: usize, pleft: usize, pright: usize) -> (usize, usize) {
            // println!("s={s:?} left={left} right={right} pleft={pleft} pright={pright}");
            if left >= right {
                return (pleft, pright);
            }
            if s[left] == s[right] {
                f(s, left + 1, right - 1, pleft, pright)
            } else {
                let (mut l, mut r) = (0, 0);
                if pleft != left {
                    let (l1, r1) = f(s, pleft, right, pleft, right);
                    if r1 - l1 > r - l {
                        l = l1;
                        r = r1;
                    }
                }
                if pright != right {
                    let (l2, r2) = f(s, left, pright, left, pright);
                    if r2 - l2 > r - l {
                        l = l2;
                        r = r2;
                    }
                }
                if left == pleft && right == pright {
                    let (l3, r3) = f(s, left + 1, right, left + 1, right);
                    if r3 - l3 > r - l {
                        l = l3;
                        r = r3;
                    }
                    let (l3, r3) = f(s, left, right - 1, left, right - 1);
                    if r3 - l3 > r - l {
                        l = l3;
                        r = r3;
                    }
                }
                (l, r)
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
    //
    // Time Complexity: O(n^3)
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
