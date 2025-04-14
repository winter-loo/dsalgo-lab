pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        
        for i in 0..=n {
            // Current height (0 when we're past the end of the array)
            let current_height = if i == n { 0 } else { heights[i] };
            
            // While the current bar is shorter than the bar at the top of the stack
            // we pop and calculate area with the popped bar as the shortest bar
            while !stack.is_empty() && current_height < heights[stack[stack.len() - 1]] {
                let height = heights[stack.pop().unwrap()];
                println!("Popped height: {}", height);
                
                // Width is current position minus the previous position in stack minus 1
                // If stack is empty, width is just the current position
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack[stack.len() - 1] - 1) as i32
                };
                
                // Calculate area and update max_area
                let area = height * width;
                max_area = max_area.max(area);
            }
            
            // Push current position onto the stack
            if i < n {
                stack.push(i);
            }
        }
        
        max_area
    }

    pub fn largest_rectangle_area_partition(heights: Vec<i32>) -> i32 {
        partition(&heights[..])
    }

    pub fn largest_rectangle_area_brute_force(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 0..heights.len() {
            for j in i..heights.len() {
                let m = heights[i..=j].iter().min().unwrap_or(&0);
                let area = m * ((j - i + 1) as i32);
                max_area = max_area.max(area);
            }
        }
        max_area
    }
}

// O(nlog(n))
pub fn partition(heights: &[i32]) -> i32 {
    if heights.is_empty() {
        return 0;
    }
    if heights.len() == 1 {
        return heights[0];
    }
    let m = heights.iter().enumerate().min_by_key(|x| x.1).map(|(i, x)| (i, *x)).unwrap_or((0, 0));
    let mut area = heights[m.0] * (heights.len() as i32);
    area = area.max(partition(&heights[0..m.0]));
    area = area.max(partition(&heights[m.0 + 1..]));
    area
}
