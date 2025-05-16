pub struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut count = 0;
        let mut min_count = i32::MAX;
        coins.sort_unstable_by(|a, b| b.cmp(a)); // decrement
        backtrack(&coins, 0, amount, &mut count, &mut min_count);
        if min_count == i32::MAX {
            -1
        } else {
            min_count
        }
    }
}

// [1, 3, 4, 5] requires to find all paths leading to zero and use the minimum
// count coins as the optimized path
fn backtrack(
    coins: &[i32],
    index: usize,
    amount: i32,
    count: &mut i32,
    min_count: &mut i32,
) -> bool {
    println!(
        "coin={} amount={amount} count={} min_count={}",
        coins[index], *count, *min_count
    );
    if amount == 0 {
        return true;
    }
    for i in index..coins.len() {
        let left = amount - coins[i];
        if left < 0 {
            break;
        }
        *count += 1;
        if backtrack(coins, index, left, count, min_count) {
            if *count < *min_count {
                println!("update min_count to {}", *min_count);
                *min_count = *count;
            }
        }
        *count -= 1;
    }
    false
}
