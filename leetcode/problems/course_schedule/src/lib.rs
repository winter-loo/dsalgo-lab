pub struct Solution;

impl Solution {
    // Time Limit Exceeded
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjlist = vec![vec![]; num_courses as usize];
        for v in prerequisites {
            adjlist[v[1] as usize].push(v[0]);
        }

        for i in 0..num_courses {
            let mut seen = vec![false; num_courses as usize];
            if dfs(&adjlist, i, &mut seen) {
                return false;
            }
        }
        true
    }
}

// detect cycle
fn dfs(adjlist: &[Vec<i32>], node: i32, seen: &mut [bool]) -> bool {
    // println!("node={node}, seen={seen:?}");
    if seen[node as usize] {
        return true;
    }

    seen[node as usize] = true;

    for neibor in adjlist[node as usize].iter() {
        if dfs(adjlist, *neibor, seen) {
            return true;
        }
    }

    seen[node as usize] = false;

    false
}
