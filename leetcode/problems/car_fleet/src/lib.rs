pub struct Solution;

impl Solution {
    /// Calculates the number of car fleets that will reach the target.
    /// 
    /// A car fleet is a group of cars that arrive at the target at the same time.
    /// Cars cannot pass each other - if a faster car catches up to a slower car,
    /// it will slow down and follow at the slower car's speed.
    /// 
    /// # Arguments
    /// * `target` - The target distance
    /// * `position` - The initial positions of the cars
    /// * `speed` - The speeds of the cars
    /// 
    /// # Returns
    /// The number of car fleets that will reach the target
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // Create a vector of (position, time to reach target) pairs
        let mut cars: Vec<(i32, f64)> = position.iter()
            .zip(speed.iter())
            .map(|(&pos, &spd)| {
                // Calculate time to reach target: (target - position) / speed
                let time = (target - pos) as f64 / spd as f64;
                (pos, time)
            })
            .collect();
        
        // Sort cars by position in descending order
        // This way we process cars from closest to target to furthest
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        
        let mut fleets = 0;
        let mut max_time = 0.0;
        
        // Process each car
        for (_, time) in cars {
            // If this car takes longer to reach the target than any car ahead of it,
            // it will form a new fleet
            if time > max_time {
                fleets += 1;
                max_time = time;
            }
            // Otherwise, it will catch up to the car ahead and join that fleet
        }
        
        fleets
    }
}
