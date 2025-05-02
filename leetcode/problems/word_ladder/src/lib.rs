pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut adjlist = vec![vec![]; word_list.len()];
        for i in 0..word_list.len() {
            for j in 0..word_list.len() {
                if i != j && 1 == distance(&word_list[i], &word_list[j]) {
                    adjlist[i].push(j);
                }
            }
        }
        // println!("adjlist={adjlist:?}");

        if word_list.iter().all(|word| *word != end_word) {
            return 0;
        }

        //
        // breadth-first search
        //
        use std::collections::VecDeque;
        let mut myq = VecDeque::new();
        for (i, word) in word_list.iter().enumerate() {
            if 1 == distance(&begin_word, word) {
                myq.push_back(i);
            }
        }
        let mut depth = if myq.is_empty() { 0 } else { 1 };
        let mut visited = vec![false; word_list.len()];
        let mut found = false;
        'qloop: while !myq.is_empty() {
            depth += 1;
            for _ in 0..myq.len() {
                if let Some(i) = myq.pop_front() {
                    visited[i] = true;
                    if word_list[i] == end_word {
                        found = true;
                        break 'qloop;
                    }
                    for neibor in adjlist[i].iter() {
                        if !visited[*neibor] {
                            myq.push_back(*neibor);
                        }
                    }
                }
            }
        }
        if found {
            depth
        } else {
            0
        }
    }
}

fn distance(a: &str, b: &str) -> u32 {
    a.chars()
        .zip(b.chars())
        .map(|(i, j)| if i == j { 0 } else { 1 })
        .sum()
}
