pub struct Solution;

impl Solution {
    // [1, 2, 3, 6, 2, 3, 4, 7, 8]
    // First intuition: count frequencies with order
    //
    // 1    2   3   4   6   7   8
    // ↓    ↓   ↓   ↓   ↓   ↓   ↓
    // 1    2   2   1   1   1   1
    //
    // 2   3   4   6   7   8
    // ↓   ↓   ↓   ↓   ↓   ↓    ──▶ 1 2 3
    // 1   1   1   1   1   1
    //
    // 6   7   8
    // ↓   ↓   ↓    ──▶ 2 3 4
    // 1   1   1
    //
    // ──▶ 6 7 8
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        use std::collections::BTreeMap;

        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut freq = BTreeMap::new();
        for n in hand {
            *freq.entry(n).or_insert(0) += 1;
        }
        println!("initial: {freq:?}");

        while !freq.is_empty() {
            let (&smallest, _) = freq.iter().next().unwrap();
            println!("smallest: {smallest}");
            for i in smallest..smallest + group_size {
                if let Some(v) = freq.get_mut(&i) {
                    *v -= 1;
                    if *v == 0 {
                        freq.remove(&i);
                    }
                } else {
                    return false;
                }
            }

            println!("updated: {freq:?}");
        }

        true
    }
}
