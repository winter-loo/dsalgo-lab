pub struct Solution;

impl Solution {
    // Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
    // 9,9,4
    // 6,6,8
    // 2,1,1
    //
    // First thought, use BFS to explore every path. O((m*n)^2)
    //
    // (0,0) -> (0,1) ❌
    // (0,0) -> (1,0) ❌
    //
    // (0,1) -> (0,0) ❌
    // (0,1) -> (0,2) ❌
    // (0,1) -> (1,1) ❌
    //
    // (0,2) -> (0,1) ✅
    // (0,2) -> (1,2) ✅
    //
    // (1,0) -> (0,0) ✅
    // (1,0) -> (1,1) ❌
    // (1,0) -> (2,0) ❌
    //
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}
