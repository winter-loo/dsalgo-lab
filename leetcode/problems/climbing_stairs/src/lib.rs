pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // f(n) = f(n - 1) + f(n - 2)
        if n <= 2 {
            return n
        }
        // let (mut f1, mut f2) = (1, 2);
        // let mut ans = 0;
        // for _ in 3..=n {
        //     ans = f1 + f2; 
        //     f1 = f2;
        //     f2 = ans;
        // }
        // ans
        (3..=n).fold((1, 2, 0), |acc, _| {
            (acc.1, acc.0 + acc.1, acc.0 + acc.1)
        }).2
    }
}
