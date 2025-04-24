pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrom(s: &[u8]) -> bool {
            let right = if s.len() % 2 == 0 {
                s.len() / 2
            } else {
                s.len() / 2 + 1
            };
            s[0..s.len() / 2]
                .iter()
                .zip(s[right..].iter().rev())
                .all(|(f, b)| f == b)
        }
        fn part(index: usize, s: &[u8], split: &mut Vec<usize>, results: &mut Vec<Vec<String>>) {
            // println!("{split:?}");
            let mut invalid_partition = false;
            let mut partition = vec![];
            for i in 0..split.len() {
                let substring = if i + 1 < split.len() {
                    &s[split[i]..split[i + 1]]
                } else {
                    &s[split[i]..]
                };
                if !is_palindrom(substring) {
                    invalid_partition = true;
                    break;
                }
                partition.push(String::from_utf8(substring.to_vec()).unwrap());
            }
            if !invalid_partition {
                results.push(partition);
            }
            for i in index..s.len() - 1 {
                split.push(i + 1);
                part(i + 1, s, split, results);
                split.pop();
            }
        }

        let mut split = vec![0];
        let mut results = vec![];
        part(0, s.as_bytes(), &mut split, &mut results);
        results
    }
}
