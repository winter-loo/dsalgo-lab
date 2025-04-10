#![allow(dead_code)]

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let v = permutation(&mut s1.chars().collect::<Vec<_>>());
        println!("v={v:?}");
        let s: HashSet<_> = v.iter().collect();
        println!("s={s:?}, len={}", s.len());
        true
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
    seq: &'a mut [char],
    _l: usize,  // Used only for API compatibility
    _r: usize,  // Used only for API compatibility
) -> impl Iterator<Item = String> + 'a {
    let n = seq.len();
    
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
struct PermutationIter<'a> {
    seq: &'a mut [char],
    // Tracks the state of the algorithm for each level
    state: Vec<usize>,
    // Size of the sequence
    n: usize,
    // Flag for the first permutation
    first: bool,
    // Flag to indicate when we're done
    done: bool,
}

impl<'a> Iterator for PermutationIter<'a> {
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


// impl<'a> Iterator for PermutationIter<'a> {
//     type Item = String;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         let l = self.lefts.last().copied().unwrap_or(0);
//         if self.i >= l || !self.indicies.is_empty() {
//             if self.i < l {
//                 println!("iteration done, return to upper level");
//                 loop {
//                     self.lefts.pop();
//                     self.i = self.indicies.pop().unwrap();
//                     println!("start from previous i={}", self.i);
//                     if self.i > l || self.indicies.is_empty() {
//                         break;
//                     } else {
//                         println!("indicies: {:?}", self.indicies);
//                     }
//                 }
// 
//                 let n = self.r - l + 1;
//                 println!("swapping where n={} i={} l={}", n, self.i, l);
//                 if n % 2 == 0 {
//                     self.seq.swap(self.i, l);
//                 } else {
//                     self.seq.swap(self.r, l);
//                 }
// 
//                 self.i = self.i.saturating_sub(1);
//                 println!("next i={}", self.i);
//                 if n == self.seq.len() && self.i == 0 {
//                     println!("whole done, indicies={:?}", self.indicies);
//                     return None;
//                 }
//             }
//             println!("go down to single element");
//             while let Some(l) = self.lefts.last().copied() {
//                 if l == self.r {
//                     break;
//                 }
//                 // save current position
//                 self.indicies.push(self.i);
//                 // to lower level
//                 self.lefts.push(l + 1);
//                 // start from new left
//                 self.i = self.r;
//             }
//             // reached to the single element, time to output current permutation
//             if let Some(top) = self.lefts.last().copied() {
//                 // save current permutation
//                 let item = self.seq.iter().collect();
//                 println!("l={} r={} s={}", top, self.r, item);
//                 // to upper level
//                 self.lefts.pop();
//                 // restore current position
//                 self.i = self.indicies.pop().unwrap();
//                 // get current left
//                 let l = self.lefts.last().copied().unwrap();
// 
//                 let n = self.r - l + 1;
//                 println!("on swapping where n={} i={} l={}", n, self.i, l);
//                 if n % 2 == 0 {
//                     self.seq.swap(self.i, l);
//                 } else {
//                     self.seq.swap(self.r, l);
//                 }
// 
//                 self.i = self.i.saturating_sub(1);
//                 println!("on next i={}", self.i);
//                 return Some(item);
//             }
//         }
//         None
//     }
// }

struct PermutationIterOpt<'a> {
    seq: &'a mut [char],
    lefts: Vec<usize>,
    indicies: Vec<usize>,
    r: usize,
    i: usize,
}

impl<'a> PermutationIterOpt<'a> {
    fn handle_level_transition(&mut self) -> bool {
        // Move up one level
        self.lefts.pop();
        self.i = self.indicies.pop().unwrap();
        
        // Get current state
        let l = self.lefts.last().copied().unwrap();
        let n = self.r - l + 1;
        
        // Perform swap based on parity
        if n % 2 == 0 {
            self.seq.swap(self.i, self.r);
        } else {
            self.seq.swap(0, self.r);
        }
        
        self.i += 1;
        n == self.seq.len() && self.i == self.seq.len()
    }
}

impl<'a> Iterator for PermutationIterOpt<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i <= self.r || !self.indicies.is_empty() {
            if self.i > self.r {
                if self.handle_level_transition() {
                    return None;
                }
            }

            // Go down to single element
            while let Some(l) = self.lefts.last().copied() {
                if l == self.r {
                    break;
                }
                self.indicies.push(self.i);
                self.lefts.push(l + 1);
                self.i = l + 1;
            }

            // Process current permutation if we've reached a leaf
            if let Some(_) = self.lefts.last().copied() {
                let item = self.seq.iter().collect();
                self.handle_level_transition();
                return Some(item);
            }
        }
        None
    }
}
