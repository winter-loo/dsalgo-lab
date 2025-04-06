# Approach: Sorting and Stack

## Intuition
The key insight is that cars can only form fleets if a faster car catches up to a slower one. Since cars cannot pass each other, we need to determine when and where cars will meet to form fleets.

## Algorithm
1. Create a list of car positions and speeds as pairs (position, speed).
2. Sort the cars by their positions in descending order (from closest to the target to furthest).
3. For each car, calculate the time it will take to reach the target: time = (target - position) / speed.
4. Process the cars in order of decreasing position:
   - If a car takes longer to reach the target than the car in front of it, it will never catch up and will form its own fleet.
   - If a car would reach the target faster than the car in front of it, it will catch up and join that fleet, moving at the slower speed.
5. Count the number of fleets that form.

## Complexity Analysis
- Time Complexity: O(n log n) where n is the number of cars
  - Sorting the cars takes O(n log n) time
  - Processing each car takes O(n) time
  
- Space Complexity: O(n)
  - We need to store the position-speed pairs and the stack of arrival times

## Implementation Notes
- Sort the cars by position in descending order
- Calculate the time to reach the target for each car
- Use a stack to keep track of the slowest car in each fleet
- A new fleet forms when a car takes longer to reach the target than the car ahead of it
