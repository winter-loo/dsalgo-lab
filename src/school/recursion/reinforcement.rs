#![allow(dead_code)]

// maxmium of sequence in recursive fashion
pub fn maxmium_of_sequence(seq: &[i32]) -> Option<i32> {
    match seq.len() {
        0 => None,
        1 => Some(seq[0]),
        _ => Some(std::cmp::max(
            seq[0],
            maxmium_of_sequence(&seq[1..]).unwrap(),
        )),
    }
}

pub fn harmonic_number(n: u32) -> f32 {
    assert!(n >= 1);
    match n {
        1 => 1.0,
        i => (1.0 / i as f32) + harmonic_number(n - 1),
    }
}

// f(13531), 1 + f(1353) * 10
// f(1353), 3 + f(135) * 10
// f(135), 5 + f(13) * 10
// f(13), 3 + f(1) * 10
// f(1), 1
//
// 3 + 1 * 10, 13
// 5 + 13 * 10, 135
// 3 + 135 * 10, 1353
// 1 + 1353 * 10, 13531
//
// benchmark shows that it's a slower function
pub fn to_u32_from_right(s: &str) -> u32 {
    fn last_int(s: &str) -> u32 {
        s.chars().last().unwrap().to_digit(10).unwrap()
    }
    match s.len() {
        0 => panic!("empty string"),
        1 => last_int(&s),
        _ => to_u32_from_right(&s[0..s.len() - 1]) * 10 + last_int(&s),
    }
}

// f(13531), 1 * pow(10, len(3531)) + f(3531)
// f(3531), 3 * pow(10, len(531)) + f(531)
// f(531), 5 * pow(10, len(31)) + f(31)
// f(31), 3 * pow(10, len(1)) + f(1)
// f(1), 1
//
// 3 * 10 + 1 = 31
// 5 * 100 + 31 = 531
// 3 * 1000 + 531 = 3531
// 1 * 10000 + 3531 = 13531
pub fn to_u32_from_left(s: &str) -> u32 {
    fn next_int(s: &str) -> u32 {
        s.chars().next().unwrap().to_digit(10).unwrap()
    }
    match s.len() {
        0 => panic!("empty string"),
        1 => next_int(&s),
        _ => next_int(&s) * 10u32.pow((s.len() - 1) as u32) + to_u32_from_left(&s[1..]),
    }
}

pub fn min_max_sequence(seq: &[i32]) -> Option<(i32, i32)> {
    match seq.len() {
        0 => None,
        1 => Some((seq[0], seq[0])),
        _ => {
            let mm = min_max_sequence(&seq[1..]);
            mm.map(|(mi, ma)| (std::cmp::min(seq[0], mi), std::cmp::max(seq[0], ma)))
        }
    }
}

pub fn log2(n: u32) -> f32 {
    if n <= 1 { 0.0 } else { 1.0 + log2(n / 2) }
}

struct Step {
    from: u8,
    to: u8,
}

struct Peg {
    name: &'static str,
}

// abstract move top n - 1 disks from a to c,
// then move the bottom disk from a to b
// abstract move n - 1 disks from c to a,
// then we a subproblem, i.e. move n - 1 disks from a to b
// the base problem is that we move 1 disk from a to b
fn hanoi_tower(n: u32, a: &mut Peg, b: &mut Peg, c: &mut Peg) {
    if n == 1 {
        println!("move from {} to {}", a.name, b.name);
        return;
    }
    hanoi_tower(n - 1, a, c, b); // abstract move top n - 1 disks from a to c
    println!("move from {} to {}", a.name, b.name); // then move the bottom disk from a to b
    #[rustfmt::skip]
    // hanoi_tower(n - 1, c, a, b); // abstract move n - 1 disks from c to a
    // hanoi_tower(n - 1, a, b, c); // then we a subproblem, i.e. move n - 1 disks from a to b
    // the above two operations can be optimized to one
    hanoi_tower(n - 1, c, b, a);
}

// f([1, 2, 3]) = {f([2, 3]), {1} + f([2, 3])}
// f([2, 3]) = {f([3]), {2} + f([3])}
// f([3]) = {f([]), {3} + f([])}
// f([]) = {}
//
// f([3]) = {{}, {3} + {}} = {{}, 3}}
// f([2, 3]) = {{}, {3},  {2}, {2, 3}}
// f([1, 2, 3]) = {{}, {3}, {2}, {2, 3}, {1}, {1, 3}, {1, 2}, {1, 2, 3}}
fn subsets(n: &[u32]) -> Vec<Vec<u32>> {
    if n.is_empty() {
        vec![vec![]]
    } else {
        let mut s = subsets(&n[1..]); // {{}, {2}}
        let mut include_a_set = Vec::with_capacity(s.len());

        for subset in &s {
            let mut new_subset = subset.clone();
            new_subset.push(n[0]);
            include_a_set.push(new_subset);
        }
        s.extend(include_a_set);
        s
    }
}

fn reverse_it(s: &mut [u8]) {
    if s.len() <= 1 {
        return;
    }
    // immutable first
    let t0 = *s.first().unwrap();
    let t1 = *s.last().unwrap();
    let second_to_last = s.len() - 1;

    // then mutable
    let last = s.last_mut().unwrap();
    *last = t0;
    let first = s.first_mut().unwrap();
    *first = t1;
    reverse_it(&mut s[1..second_to_last]);
}

fn is_palindrom(s: &[u8]) -> bool {
    if s.len() <= 1 {
        return true;
    }
    return s.first().unwrap() == s.last().unwrap() && is_palindrom(&s[1..s.len() - 1]);
}

fn more_vowels_than_consonants(s: &[u8]) -> bool {
    const VOWELS: &[u8] = b"aeiouAEIOU";

    fn inner(s: &[u8], count: &mut (u32, u32)) {
        if !s.is_empty() {
            inner(&s[1..], count);
            if VOWELS.contains(s.first().unwrap()) {
                count.0 += 1;
            } else {
                count.1 += 1;
            }
        }
    }

    let mut count = (0, 0);
    inner(s, &mut count);
    count.0 > count.1
}

// rearranges a sequence of integer values so that all the even values appear before
// all the odd values.
fn rearrange_even_before_odd(seq: &mut [i32]) {
    if seq.is_empty() {
        return;
    }
    let last = seq.len() - 1;
    // if the last number is even, swap it with the first, then our problem
    // becomes smaller by excluding the first number
    //
    // if the last number is odd, we know it's already in its position, then
    // our problem becomes smaller by excluding the last number
    if seq.last().unwrap() % 2 == 0 {
        seq.swap(0, last);
        rearrange_even_before_odd(&mut seq[1..]);
    } else {
        rearrange_even_before_odd(&mut seq[0..last]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maxmium_of_sequence() {
        assert_eq!(maxmium_of_sequence(&[]), None);
        assert_eq!(maxmium_of_sequence(&[1, 2, 3]), Some(3));
        assert_eq!(maxmium_of_sequence(&[2, 1, 3]), Some(3));
        assert_eq!(maxmium_of_sequence(&[3, 2, 1]), Some(3));
    }

    #[test]
    fn test_harmonic_number() {
        fn are_floats_equal(x: f32, y: f32) -> bool {
            (x - y).abs() < 1e-8
        }

        assert!(are_floats_equal(harmonic_number(1), 1f32));
        assert!(are_floats_equal(harmonic_number(2), 1.5f32));
        assert!(are_floats_equal(harmonic_number(3), 1.8_333_334f32));
    }

    #[test]
    fn test_to_u32_from_right() {
        assert_eq!(to_u32_from_right("0"), 0);
        assert_eq!(to_u32_from_right("1"), 1);
        assert_eq!(to_u32_from_right("12345"), 12345);
    }

    #[test]
    fn test_to_u32_from_left() {
        assert_eq!(to_u32_from_left("0"), 0);
        assert_eq!(to_u32_from_left("1"), 1);
        assert_eq!(to_u32_from_left("12345"), 12345);
    }

    #[test]
    fn test_min_max_sequence() {
        assert_eq!(min_max_sequence(&[1, 5, 3, 2]), Some((1, 5)));
        assert_eq!(min_max_sequence(&[]), None);
    }

    #[test]
    fn test_log2() {
        assert_eq!(log2(0) as u32, 0);
        assert_eq!(log2(1) as u32, 0);
        assert_eq!(log2(2) as u32, 1);
        assert_eq!(log2(3) as u32, 1);
        assert_eq!(log2(4) as u32, 2);
    }

    #[test]
    fn test_hanoi_tower() {
        let mut a = Peg { name: "a" };
        let mut b = Peg { name: "b" };
        let mut c = Peg { name: "c" };
        hanoi_tower(3, &mut a, &mut b, &mut c);
    }

    #[test]
    fn test_subsets() {
        let all = subsets(&[3, 2, 1]);
        assert_eq!(all.len(), 2usize.pow(3));
    }

    #[test]
    fn test_reverse_it() {
        // b"pots&pans" is a '&[u8; 9]'
        // we want '&mut [u8]'
        // first, deference it [u8; 9] owned by 's', also make it mutable
        let mut s = *b"pots&pans";
        // mutable borrow it
        reverse_it(&mut s);
        // compare two '&[u8]'
        assert_eq!(&s, b"snap&stop");
    }

    #[test]
    fn test_is_palindrom() {
        assert!(is_palindrom(b"madam"));
        assert!(!is_palindrom(b"madamw"));
        assert!(is_palindrom(b""));
        assert!(is_palindrom(b"t"));
    }

    #[test]
    fn test_more_vowels_than_consonants() {
        assert!(!more_vowels_than_consonants(b"")); // Empty string should be false
        assert!(!more_vowels_than_consonants(b"t")); // Single consonant
        assert!(more_vowels_than_consonants(b"a")); // Single vowel
        assert!(!more_vowels_than_consonants(b"b")); // Single consonant
        assert!(!more_vowels_than_consonants(b"ab")); // Equal vowels and consonants
        assert!(more_vowels_than_consonants(b"ae")); // More vowels
        assert!(more_vowels_than_consonants(b"aeiouaei")); // More vowels
        assert!(!more_vowels_than_consonants(b"bcdfg")); // All consonants
        assert!(more_vowels_than_consonants(b"aeiouAEI")); // Mixed case
    }

    #[test]
    fn test_rearrange_even_before_odd() {
        let mut s = [3, 2, 1];
        rearrange_even_before_odd(&mut s);
        assert_eq!(s, [2, 3, 1]);
        let mut s1 = [4, 3, 2, 1];
        rearrange_even_before_odd(&mut s1);
        assert_eq!(s1, [2, 4, 3, 1]);

        let mut s2 = [2, 4, 6, 8];
        rearrange_even_before_odd(&mut s2);
        assert_eq!(s2, [8, 2, 4, 6]);

        let mut s3 = [1, 3, 5, 7];
        rearrange_even_before_odd(&mut s3);
        assert_eq!(s3, [1, 3, 5, 7]);

        let mut s4 = [];
        rearrange_even_before_odd(&mut s4);
        assert_eq!(s4, []);

        let mut s5 = [0, 1, 2, 3, 4, 5];
        rearrange_even_before_odd(&mut s5);
        assert_eq!(s5, [4, 0, 2, 3, 1, 5]);

        let mut s6 = [1];
        rearrange_even_before_odd(&mut s6);
        assert_eq!(s6, [1]);

        let mut s7 = [2];
        rearrange_even_before_odd(&mut s7);
        assert_eq!(s7, [2]);

    }
}
