pub struct Solution;

impl Solution {
    // Input: nums = [3,1,5,8]
    // Still a decision/selection problem.
    // Initially, you have 4 choices, then 3, then 2, then 1.
    // Hence, we can use decision tree to solve this problem.
    //
    //  Decision tree here:
    // 'https://excalidraw.com/#json=mDsKWddH5cPdlebx29tQ-,qpZEGZW0Kj6EXOOh9YeMAQ'
    //
    // From the above tree, we can get some conclusions:
    // - when there is only 1 balloon, the max coins you can get is the number in it.
    // - when there are 2 balloons, always choose the smaller one first
    // - when there are 3 balloons, ...
    // - when there are 4 balloons, ...
    //
    //              3       1       5       8 
    //            ┌─────────────────────────── 
    //        3 1 │ 3       1       5       8
    //        1 2 │          
    //        5 3 │                  
    //        8 4 │                          
    //
    //  It's hard to think like that. Maybe we need try other methods.
    //
    //  In previous logic, we try to go from one end to the other end. We could
    //  switch logic, how about from the middle to both sides.
    //
    //  3 (1 5) 8 ┌─choose 1──> 3 (5) 8, we got 15 ──choose 5──> 3 8, we got 135
    //            └─choose 5──> 3 (1) 8, we got 50 ──choose 1──> 3 8, we got 74
    //
    //  Pattern = A (M) B
    //  M can contain many numbers and will aggregate into only one number.
    //  Hence, M is the subproblem.
    //  Say, A is '3', M is '1 5 8', B is '8'
    //  For M='1 5 8' ──5──> 1 8, we got 40 ────> 16 + 40 = 56
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        todo!()
    }
}
