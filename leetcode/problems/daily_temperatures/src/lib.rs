pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut descreasing_stack = vec![];
        let mut answer = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while descreasing_stack.last().is_some_and(|x| temperatures[*x] < temperatures[i]) {
                let n = descreasing_stack.pop().unwrap();
                answer[n] = (i - n) as i32;
            }
            descreasing_stack.push(i);
        }
        answer
    }

    pub fn daily_temperatures_initial(temperatures: Vec<i32>) -> Vec<i32> {
        if temperatures.is_empty() {
            return vec![];
        }
        if temperatures.len() == 1 {
            return vec![0];
        }

        // iterate the array and put undetermined elements into it
        // the undetermined elements have a decreasing order
        let mut unsolved = vec![];
        let mut result = vec![0i32; temperatures.len()];
        for i in 0..temperatures.len() {
            while unsolved.last().is_some_and(|x| temperatures[*x] < temperatures[i]) {
                let n = unsolved.pop().unwrap();
                result[n] = (i - n) as i32;
            }
            if i + 1 < temperatures.len() && temperatures[i] < temperatures[i + 1] {
                result[i] = 1;
            } else if i + 1 == temperatures.len() {
                result[i] = 0;
            } else {
                unsolved.push(i);
            }
        }
        result
    }
}
