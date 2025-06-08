pub struct Solution;

impl Solution {
    // 1,1,1,1,1
    //
    // Here's thinking: '+' and '-' is the equivalent to 'select' and 'not select'.
    // So, '+' means select current element; '-' means not select current element.
    //
    // S=1,1,1,1,1 T=3
    // f(t,i) = f(t-S[i],i+1) + f(t+S[i],i+1) for i in [0,len(S)] and target is 't'.
    // or
    // f(t,i) = f(t+S[i-1],i-1) + f(t-S[i-1],i-1), t-S[i] < t < t+S[i]
    //
    // S\T      0   1   2   3   4
    //      +----------------
    //   0  |   1   0   0   0   0
    // 1 1  |   0   1   0   0   0
    // 1 2  |   1   0   1   0   0
    // 1 3  |   0   1   0   1
    // 1 4  |       0
    // 1 5  |       0
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}
