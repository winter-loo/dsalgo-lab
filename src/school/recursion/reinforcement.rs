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
    if n <= 1 {
        0.0
    } else {
        1.0 + log2(n / 2)
    }
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
}
