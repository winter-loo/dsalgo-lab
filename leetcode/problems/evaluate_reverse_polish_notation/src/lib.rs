pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for s in tokens {
            match &s[..] {
                "+" => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 + op1);
                }
                "-" => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 - op1);
                }
                "*" => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 * op1);
                }
                "/" => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 / op1);
                }
                _ => {
                    let operand = s.parse::<i32>().unwrap();
                    stack.push(operand);
                }
            }
        }
        stack.last().copied().unwrap()
    }
}
