pub struct Solution;

impl Solution {
    // 1,1,1,1,1
    //
    // Here's thinking: '+' and '-' is the equivalent to 'select' and 'not select'.
    // So, '+' means select current element; '-' means not select current element.
    //
    // S=1,1,1,1,1 T=3
    // f(t,i) = f(t-S[i],i+1) + f(t+S[i],i+1) for i in [0,len(S)] and target is 't'.
    // The answer is `f(T,0)` which means staring from the index 0 of S, the number
    // of ways to sum to T.
    //
    // S\T   -1 | 0   1   2   3 | 4
    //      +---|---------------|--
    // 1 5  | 0 | 0   1   0   0 | 0
    // 1 4  | 0 | 1   0   1   0 | 0
    // 1 3  | 0 | 0   2   0   1 | 0
    // 1 2  | 0 | 2   0   3   0 | 0
    // 1 1  | 0 | 0   5   0   3 | 0
    //
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}
