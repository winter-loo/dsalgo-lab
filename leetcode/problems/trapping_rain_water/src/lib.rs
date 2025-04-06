pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        Solution::trap_stack(height)
    }

    // https://excalidraw.com/#json=451Hk31dL4NbYuzvAQLCh,cij2ouokfIklaHaCsuuqQA
    // Stack-based approach - O(n) time, O(n) space
    pub fn trap_stack(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut stack: Vec<usize> = Vec::new(); // Stack to store indices
        let mut water = 0;

        // Process each bar
        for current in 0..height.len() {
            // While current bar is higher than the bar at top of stack
            while !stack.is_empty() && height[current] > height[*stack.last().unwrap()] {
                let top = stack.pop().unwrap(); // Pop the top element

                // If stack becomes empty, no water can be trapped
                if stack.is_empty() {
                    break;
                }

                // Calculate width between current bar and the bar at stack top
                let distance = current as i32 - *stack.last().unwrap() as i32 - 1;

                // Calculate bounded height (min of current and stack top, minus the popped height)
                let bounded_height =
                    (height[current].min(height[*stack.last().unwrap()]) - height[top]).max(0);

                // Add water trapped in this section
                water += distance * bounded_height;
            }

            // Push current bar's index to stack
            stack.push(current);
        }

        water
    }

    // Two pointers approach - O(n) time, O(1) space
    pub fn trap_two_pointers(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                // If current height is greater than left_max, update left_max
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    // Otherwise, we can trap water at this position
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                // If current height is greater than right_max, update right_max
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    // Otherwise, we can trap water at this position
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
    }

    // Precomputation approach - O(n) time, O(n) space
    pub fn trap_precomputation(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut left_maxs = vec![0; height.len()];
        let mut right_maxs = vec![0; height.len()];

        let mut curr_max = 0;
        for i in 0..height.len() {
            left_maxs[i] = curr_max;
            if curr_max < height[i] {
                curr_max = height[i];
            }
        }

        let mut curr_max = 0;
        for i in (0..height.len()).rev() {
            right_maxs[i] = curr_max;
            if curr_max < height[i] {
                curr_max = height[i];
            }
        }

        let mut sum = 0;
        for i in 0..height.len() {
            let m = left_maxs[i].min(right_maxs[i]);
            if m < height[i] {
                continue;
            }
            sum += m - height[i];
        }
        sum
    }

    // Brute force approach - O(n²) time, O(1) space
    pub fn trap_brute_force(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut sum = 0;
        for i in 1..height.len() - 1 {
            let lm = (0..=i - 1).map(|n| height[n]).max().unwrap_or(0);
            if lm < height[i] {
                continue;
            }

            let rm = (i + 1..height.len()).map(|n| height[n]).max().unwrap_or(0);
            if rm < height[i] {
                continue;
            }

            // println!("lm={lm} rm={rm} i={i} height={}", height[i]);
            sum += lm.min(rm) - height[i];
        }
        sum
    }
}
