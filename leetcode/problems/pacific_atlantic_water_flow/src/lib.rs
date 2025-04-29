pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(!heights.is_empty() && !heights[0].is_empty());
        let (rows, cols) = (heights.len(), heights[0].len());
        let mut result = vec![];
        for row in 0..rows {
            for col in 0..cols {
                let (row, col) = (row as i32, col as i32);
                if could_flow_to_pacific(&heights, (row, col))
                    && could_flow_to_atlantic(&heights, (row, col))
                {
                    result.push(vec![row, col]);
                }
            }
        }
        result
    }
}

#[inline]
fn could_flow_to_pacific(heights: &[Vec<i32>], point: (i32, i32)) -> bool {
    let mut visited = vec![false; heights.len() * heights[0].len()];
    // println!("could_flow_to_pacific");
    could_flow_to_the_ocean(heights, i32::MAX, point, &mut visited, &|(row, col)| {
        (
            row < 0 || col < 0,
            row >= heights.len() as i32 || col >= heights[0].len() as i32,
        )
    })
}

#[inline]
fn could_flow_to_atlantic(heights: &[Vec<i32>], point: (i32, i32)) -> bool {
    let mut visited = vec![false; heights.len() * heights[0].len()];
    // println!("could_flow_to_atlantic");
    could_flow_to_the_ocean(heights, i32::MAX, point, &mut visited, &|(row, col)| {
        (
            row >= heights.len() as i32 || col >= heights[0].len() as i32,
            row < 0 || col < 0,
        )
    })
}

fn could_flow_to_the_ocean(
    heights: &[Vec<i32>],
    prev_value: i32,
    point: (i32, i32),
    visited: &mut [bool],
    // the first bool: reached to the ocean
    // the second bool: unable to reach to the ocean
    reached: &dyn Fn((i32, i32)) -> (bool, bool),
) -> bool {
    // println!("prev_value={prev_value}, point={point:?}");
    let term = reached((point.0, point.1));
    if term.0 {
        // println!("reached");
        return true;
    }
    if term.1  {
        // println!("impossible to reach");
        return false;
    }
    let (row, col) = (point.0 as usize, point.1 as usize);
    let idx = row * heights[0].len() + col;
    if visited[idx] {
        // println!("impossible to reach");
        return false;
    }
    if heights[row][col] > prev_value {
        // println!("impossible to reach");
        return false;
    }
    visited[idx] = true;
    let prev_value = heights[row][col];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dr, dc) in directions.iter() {
        let (next_row, next_col) = (point.0 + dr, point.1 + dc);
        if could_flow_to_the_ocean(heights, prev_value, (next_row, next_col), visited, &reached) {
            return true;
        }
    }
    false
}
