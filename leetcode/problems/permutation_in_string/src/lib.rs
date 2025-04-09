use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let v = permutation_of_heap(&mut s1.chars().collect::<Vec<_>>(), 0, s1.len() - 1);
        println!("v={v:?}");
        let s: HashSet<_> = v.iter().collect();
        println!("s={s:?}, len={}", s.len());
        true
    }
}

#[allow(dead_code)]
mod initial {
    // f(abc) = a + f(bc) | b + f(ac) | c + f(ab)
    // f(bc) = b + f(c) | c + f(b)
    // f(ac) = a + f(c) | c + f(a)
    // f(ab) = a + f(b) | b + f(a)
    fn permutation_collect(input: String) -> Vec<String> {
        assert!(input.chars().all(|c| c.is_ascii_lowercase()));

        if input.len() < 2 {
            return vec![input];
        }

        let mut input: Vec<char> = input.chars().collect();
        permutation_impl(&mut input)
    }

    fn permutation_impl(input: &mut [char]) -> Vec<String> {
        if input.len() < 2 {
            return vec![input.iter().collect()];
        }
        let mut result = vec![];
        for i in 0..input.len() {
            input.swap(0, i);
            let prefix = input[0].to_string();
            for p in permutation_impl(&mut input[1..]) {
                result.push(prefix.clone() + &p);
            }
            input.swap(0, i);
        }
        result
    }
}

/// Heap's permutation algorithm
pub fn permutation_of_heap(seq: &mut [char], l: usize, r: usize) -> Vec<String> {
    if l == r {
        let s = seq.iter().collect();
        println!("l={l} r={r} s={s}");
        return vec![s];
    }

    let n = r - l + 1;
    let mut result = Vec::with_capacity(fact(n));
    for i in l..=r {
        println!("i={i} l={l} r={r}");
        result.extend(permutation_of_heap(seq, l + 1, r));
        if n % 2 == 0 {
            seq.swap(i, r);
        } else {
            seq.swap(0, r);
        }
    }
    result
}

fn fact(n: usize) -> usize {
    (1..=n).fold(1, |p, n| p * n)
}

pub fn permutation<'a>(
    seq: &'a mut [char],
    l: usize,
    r: usize,
) -> impl Iterator<Item = String> + 'a {
    PermutationIter {
        seq,
        lefts: vec![l],
        indicies: vec![l],
        r,
        i: 0,
    }
}

struct PermutationIter<'a> {
    seq: &'a mut [char],
    lefts: Vec<usize>,
    indicies: Vec<usize>,
    r: usize,
    i: usize,
}

impl<'a> Iterator for PermutationIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i <= self.r || !self.indicies.is_empty() {
            if self.i > self.r {
                println!("iteration done, return to upper level");
                self.lefts.pop();
                self.i = self.indicies.pop().unwrap();
                println!("start from previous i={}", self.i);
                // get current left
                let l = self.lefts.last().copied().unwrap();

                let n = self.r - l + 1;
                println!("swapping where n = {} i = {}", n, self.i);
                if n % 2 == 0 {
                    self.seq.swap(self.i, self.r);
                } else {
                    self.seq.swap(0, self.r);
                }

                self.i += 1;
                println!("next i={}", self.i);
                if n == self.seq.len() && self.i == self.seq.len() {
                    println!("whole done, indicies={:?}", self.indicies);
                    return None;
                }
            }
            println!("go down to single element");
            while let Some(l) = self.lefts.last().copied() {
                if l == self.r {
                    break;
                }
                // save current position
                self.indicies.push(self.i);
                // to lower level
                self.lefts.push(l + 1);
                // start from new left
                self.i = l + 1;
            }
            // reached to the single element, time to output current permutation
            if let Some(top) = self.lefts.last().copied() {
                // save current permutation
                let item = self.seq.iter().collect();
                println!("l={} r={} s={}", top, self.r, item);
                // to upper level
                self.lefts.pop();
                // restore current position
                self.i = self.indicies.pop().unwrap();
                // get current left
                let l = self.lefts.last().copied().unwrap();

                let n = self.r - l + 1;
                println!("swapping where n={} i={}", n, self.i);
                if n % 2 == 0 {
                    self.seq.swap(self.i, self.r);
                } else {
                    self.seq.swap(0, self.r);
                }

                self.i += 1;
                println!("next i={}", self.i);
                return Some(item);
            }
        }
        None
    }
}
