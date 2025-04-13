pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        
        // Start backtracking with 0 open and 0 close parentheses
        Self::backtrack_from_ai(&mut result, &mut current, 0, 0, n);
        
        result
    }
    
    fn backtrack_from_ai(result: &mut Vec<String>, current: &mut String, open: i32, close: i32, max: i32) {
        // Base case: if the current string has 2*n characters, we've found a valid combination
        if current.len() == (max * 2) as usize {
            result.push(current.clone());
            return;
        }
        
        // We can add an opening parenthesis if we have used fewer than n
        if open < max {
            current.push('(');
            Self::backtrack_from_ai(result, current, open + 1, close, max);
            current.pop(); // Backtrack by removing the last character
        }
        
        // We can add a closing parenthesis if we have more opening than closing ones
        if close < open {
            current.push(')');
            Self::backtrack_from_ai(result, current, open, close + 1, max);
            current.pop(); // Backtrack by removing the last character
        }
    }

    pub fn generate_parenthesis_recur(n: i32) -> Vec<String> {
        let mut path = vec![];
        let mut results = vec![];
        myimpl(n, n, &mut path, &mut results);
        results
    }
}

// https://excalidraw.com/#json=o1wMG7DLevvqQ4r6IQ21R,MJyhYLgZliHiE3_oHLdd5A
// []           3
// (            2
// (,(          1
// (,(,(        0
// => ((()))
// (,(,)        1
// (,(,),(      0
// => (()())
// (,(,),)      1
// (,(,),),(    0
// => (())()
// (,(,),)
// (,(,)
// (,(
// (,)
// (,),(
// (,),(,(
// => ()(())
// (,),(,)
// (,),(,),(
// => ()()()
pub fn myimpl(n: i32, m: i32, path: &mut Vec<char>, paths: &mut Vec<String>) {
    // println!("path={path:?} n={n} m={m}");
    if n == 0 {
        let seq = path.iter().chain((0..m).map(|_| &')')).collect();
        paths.push(dbg!(seq));
        return;
    }
    path.push('(');
    myimpl(n - 1, m, path, paths);
    path.pop();
    if m > n {
        path.push(')');
        myimpl(n, m - 1, path, paths);
        path.pop();
    }
}
