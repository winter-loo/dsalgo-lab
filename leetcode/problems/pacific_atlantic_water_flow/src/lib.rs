pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(!heights.is_empty() && !heights[0].is_empty());
        let (rows, cols) = (heights.len(), heights[0].len());
        let mut result = vec![];
        for row in 0..rows {
            for col in 0..cols {
                if could_flow_to_pacific(&heights, (row, col))
                    && could_flow_to_atlantic(&heights, (row, col))
                {
                    result.push(vec![row as i32, col as i32]);
                }
            }
        }
        result
    }
}

fn could_flow_to_pacific(heights: &[Vec<i32>], point: (usize, usize)) -> bool {
    could_flow_to_ocean_by_directions(heights, point, &[(0, -1), (-1, 0)], |(row, col)| {
        row < 0 || col < 0
    })
}
fn could_flow_to_atlantic(heights: &[Vec<i32>], point: (usize, usize)) -> bool {
    could_flow_to_ocean_by_directions(heights, point, &[(0, 1), (1, 0)], |(row, col)| {
        row as usize >= heights.len() || col as usize >= heights[0].len()
    })
}
fn could_flow_to_ocean_by_directions<F>(
    heights: &[Vec<i32>],
    point: (usize, usize),
    directions: &[(isize, isize)],
    reached: F,
) -> bool
where
    F: Fn((isize, isize)) -> bool,
{
    if reached((point.0 as isize, point.1 as isize)) {
        return true;
    }
    false
}
