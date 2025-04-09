use permutation_in_string::{permutation, permutation_of_heap};

fn main() {
    let s1 = "abc".to_string();
    let mut s: Vec<char> = s1.chars().collect();

    // let v = permutation_of_heap(&mut s, 0, s1.len() - 1);
    // println!("v={v:?} len={}", v.len());

    let perm = permutation(&mut s, 0, s1.len() - 1);
    for p in perm {
        println!("p={p}");
    }
}
