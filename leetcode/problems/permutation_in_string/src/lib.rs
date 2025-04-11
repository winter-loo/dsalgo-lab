#![allow(dead_code)]

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // println!("s1={s1} s2={s2}");

        let mut m = HashMap::new();
        for c in s1.chars() {
            *m.entry(c).or_insert(0) += 1;
        }

        let (mut l, mut r) = (0, 0);
        let s2: Vec<char> = s2.chars().collect();

        let mut m2: HashMap<char, Vec<usize>> = HashMap::new();

        while r < s2.len() {
            // println!("l={l} r={r}");
            if let Some(n) = m.get_mut(&s2[r]) {
                if *n > 0 {
                    if r - l + 1 == s1.len() {
                        return true
                    }

                    m2.entry(s2[r]).or_insert(vec![]).push(r);
                    *n -= 1;
                    r += 1;
                } else {
                    let indicies = m2.get_mut(&s2[r]).unwrap();
                    l = indicies.remove(0) + 1;
                    indicies.push(r);
                    *n = indicies.len();
                }
            } else {
                m2.clear();
                r += 1;
                l = r;
            }
        }
        false
    }
}

#[allow(dead_code)]
pub mod initial {
    // f(abc) = a + f(bc) | b + f(ac) | c + f(ab)
    // f(bc) = b + f(c) | c + f(b)
    // f(ac) = a + f(c) | c + f(a)
    // f(ab) = a + f(b) | b + f(a)
    pub fn permutation_collect(input: String) -> Vec<String> {
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

pub fn permutation(seq: &mut [char]) -> Vec<String> {
    permutation_of_heap_fixed(seq, 0, seq.len() - 1)
}

pub fn permutation_of_heap_tail(seq: &mut [char], l: usize, r: usize) -> Vec<String> {
    permutation_of_heap_fixed(seq, l, r)
}

/// Heap's permutation algorithm
pub fn permutation_of_heap_fixed(seq: &mut [char], l: usize, r: usize) -> Vec<String> {
    if l == r {
        let s = seq.iter().collect();
        // println!("l={l} r={r} s={s}");
        return vec![s];
    }

    let n = r - l + 1;
    let mut result = Vec::with_capacity(fact(n));
    for i in (l..=r).rev() {
        // println!("i={i} l={l} r={r}");
        result.extend(permutation_of_heap_fixed(seq, l + 1, r));
        if n % 2 == 0 {
            seq.swap(i, l);
        } else {
            seq.swap(r, l);
        }
    }
    result
}

/// No AI can fix this with minimal changes
/// Heap's permutation algorithm
pub fn permutation_of_heap_buggy(seq: &mut [char], l: usize, r: usize) -> Vec<String> {
    if l == r {
        let s = seq.iter().collect();
        // println!("l={l} r={r} s={s}");
        return vec![s];
    }

    let n = r - l + 1;
    let mut result = Vec::with_capacity(fact(n));
    for i in l..=r {
        // println!("i={i} l={l} r={r}");
        result.extend(permutation_of_heap_buggy(seq, l + 1, r));
        if n % 2 == 0 {
            seq.swap(i, r);
        } else {
            seq.swap(l, r);
        }
    }
    result
}

// compare with the initial::permutation_collect
pub fn permutation_of_heap2(seq: &mut [char], l: usize, r: usize) -> Vec<String> {
    let mut result = Vec::new();
    
    if l == r {
        // Base case: found one permutation
        let s: String = seq.iter().collect();
        return vec![s];
    }
    
    for i in l..=r {
        // Swap elements
        seq.swap(l, i);
        
        // Recursively generate permutations with the first element fixed
        let mut perms = permutation_of_heap2(seq, l + 1, r);
        result.append(&mut perms);
        
        // Swap back to restore the original sequence
        seq.swap(l, i);
    }
    
    result
}

pub fn permutation_of_heap_head(seq: &mut [char], n: usize) -> Vec<String> {
    println!("permutation of {n}");
    let mut result = Vec::new();
    
    if n == 1 {
        let s = seq.iter().collect();
        println!("n=1 s={}", s);
        result.push(s);
        return result;
    }
    
    for i in 0..n {
        // Generate permutations with (n-1) elements
        let mut perms = permutation_of_heap_head(seq, n - 1);
        result.append(&mut perms);
        
        // If n is odd, swap first and last element
        // If n is even, swap i-th and last element
        if n % 2 == 1 {
            println!("swapping first 0 with last {} where n={n}", n - 1);
            seq.swap(0, n - 1);
        } else {
            println!("swapping i={i} with last {} where n={n}", n - 1);
            seq.swap(i, n - 1);
        }
    }
    
    println!("one iteration done where n={n}");
    result
}

fn fact(n: usize) -> usize {
    (1..=n).fold(1, |p, n| p * n)
}

pub fn permutation_iter<'a>(
    seq: String,
) -> impl Iterator<Item = String> {
    let n = seq.len();
    let seq = seq.chars().collect();
    
    PermutationIter {
        seq,
        state: vec![0; n],  // Initialize state array with zeros
        n,                  // Store sequence length
        first: true,        // First call flag
        done: false,        // Done flag
    }
}

/// A permutation iterator that uses Heap's algorithm.
/// This is an efficient non-recursive implementation that minimizes the number of swaps.
struct PermutationIter {
    seq: Vec<char>,
    // Tracks the state of the algorithm for each level
    state: Vec<usize>,
    // Size of the sequence
    n: usize,
    // Flag for the first permutation
    first: bool,
    // Flag to indicate when we're done
    done: bool,
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // If we're done, return None
        if self.done {
            return None;
        }
        
        // For the first call, just return the initial sequence without any swaps
        if self.first {
            self.first = false;
            return Some(self.seq.iter().collect());
        }
        
        // Heap's algorithm (non-recursive implementation)
        let mut i = 0;
        while i < self.n {
            if self.state[i] < i {
                // Determine which elements to swap based on parity
                let j = if i % 2 == 0 { 0 } else { self.state[i] };
                
                // Perform the swap
                self.seq.swap(j, i);
                
                // Update state and move to the next permutation
                self.state[i] += 1;
                return Some(self.seq.iter().collect());
            } else {
                // Reset state and move to the next position
                self.state[i] = 0;
                i += 1;
            }
        }
        
        // If we've gone through all permutations, we're done
        self.done = true;
        None
    }
}
