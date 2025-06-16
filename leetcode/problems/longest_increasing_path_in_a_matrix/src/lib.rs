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
    // For any position, (r,c),
    // if M(r,c-1) > M(r,c), then f(r,c) = 1 + f(r,c-1) // left
    // if M(r-1,c) > M(r,c), then f(r,c) = 1 + f(r-1,c) // top
    // if M(r,c+1) > M(r,c), then f(r,c) = 1 + f(r,c+1) // right
    // if M(r+1,c) > M(r,c), then f(r,c) = 1 + f(r+1,c) // bottom
    //
    // 1,2,3
    // 6,5,4
    // 7,8,9
    //
    // 1 ┌─> 2 ┌─> 3 ──> 4 ┌─> 5 ┌─> 6 ──> 7 ──> 8 ──> 9
    //   │     │           │     └─> 8 ──> 9
    //   │     │           └─> 9
    //   │     └─> 5 ┌─> 6 ──> 7 ──> 8 ──> 9
    //   │           └─> 8 ──> 9
    //   └─> 6 ──> 7 ──> 8 ──> 9
    //
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}
