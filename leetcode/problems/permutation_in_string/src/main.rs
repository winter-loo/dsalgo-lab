use permutation_in_string::*;
use std::collections::BTreeSet;

fn main() {
    let s1 = "abcd".to_string();
    let mut s: Vec<char> = s1.chars().collect();

    // let v = initial::permutation_collect(s1.clone());
    // println!("v={v:#?} len={}", v.len());

    // let v = permutation_of_heap_fixed(&mut s, 0, s1.len() - 1);
    // println!("v={v:#?} len={}", v.len());
    // let vs = v.iter().collect::<BtreeSet<_>>();
    // println!("vs={vs:#?} len={}", vs.len());

    let perm = permutation_iter(&mut s, 0, s1.len() - 1);
    let v: Vec<_> = perm.collect();
    for p in v.iter() {
        println!("p={p}");
    }
    let vs = v.iter().collect::<BTreeSet<_>>();
    println!("len={}", vs.len());
}
