pub struct Solution;

impl Solution {
    // Input: cost = [10,15,20]
    // Output: 15
    //
    // f(4) -> 0
    // f(3) -> 20
    // f(2) -> 15 + min(f(3), f(4)) -> 15
    // f(1) -> 10 + min(f(2), f(3)) -> 10 + 15 -> 25
    // f(0) -> 0 + min(f(1), f(2)) -> 15
    //
    // ideas come from min_cost_climbing_stairs_with_memoization:
    // current minimum cost always depends on future costs
    pub fn min_cost_climbing_stairs(costs: Vec<i32>) -> i32 {
        if costs.len() < 2 {
            return 0;
        }

        let mut n1 = 0;
        let mut n2 = *costs.last().unwrap_or(&0);
        for i in (0..costs.len() - 1).rev() {
            let ans = costs[i] + n1.min(n2);
            n1 = n2;
            n2 = ans;
        }
        n1.min(n2)
    }

    pub fn min_cost_climbing_stairs_with_memoization(costs: Vec<i32>) -> i32 {
        // decision tree with memoization
        fn choose(costs: &[i32], index: isize, mem: &mut [i32]) -> i32 {
            if index >= costs.len() as isize {
                return 0;
            }
            let next_index = (index + 1) as usize;
            if mem[next_index] != i32::MAX {
                return mem[next_index];
            }
            let left_cost = choose(costs, index + 1, mem) + costs.get(next_index).unwrap_or(&0);
            let right_cost = choose(costs, index + 2, mem) + costs.get(next_index + 1).unwrap_or(&0);
            mem[next_index] = left_cost.min(right_cost);
            mem[next_index]
        }
        let mut mem = vec![i32::MAX; costs.len() + 1];
        // -1 -> no decision done yet
        choose(&costs, -1, &mut mem)
    }

    // see https://excalidraw.com/#json=yP7yj0Wd4-Ryo7RdjZ50x,ud1_H_5bUSeUd3yVXiHxeg
    pub fn min_cost_climbing_stairs_brute_force(costs: Vec<i32>) -> i32 {
        // Time Limit Exceeded
        // decision tree without pruning
        fn choose(costs: &[i32], index: isize) -> i32 {
            if index >= costs.len() as isize {
                return 0;
            }
            let left_cost = choose(costs, index + 1) + costs.get((index + 1) as usize).unwrap_or(&0);
            let right_cost = choose(costs, index + 2) + costs.get((index + 2) as usize).unwrap_or(&0);
            left_cost.min(right_cost)
        }
        // -1 -> no decision done yet
        choose(&costs, -1)
    }
}
