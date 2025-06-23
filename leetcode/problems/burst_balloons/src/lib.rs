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
    // - when there are 2 balloons, always choose the smaller one
    // - when there are 3 balloons, ...
    // - when there are 4 balloons, ...
    //
    //              3       1       5       8 
    //            ┌─────────────────────────── 
    //          3 │ 3       
    //          1 │         1
    //          5 │                 5
    //          8 │                         8
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        todo!()
    }
}
