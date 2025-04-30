pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        Solution::can_finish_topological_sort(num_courses, prerequisites)
    }

    pub fn can_finish_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut adjlist = vec![vec![]; num_courses];
        for v in prerequisites {
            adjlist[v[1] as usize].push(v[0] as usize);
        }
        // println!("adjlist={adjlist:?}");

        let mut in_degrees = vec![0usize; num_courses];
        for neibors in adjlist.iter() {
            for nb in neibors {
                in_degrees[*nb] += 1;
            }
        }
        // println!("in_degrees={in_degrees:?}");

        // This idea, i.e. starting from in_degress=0, is also embodied in the
        // problem 'Surrounded Regions' and 'Pacific Atlantic Water Flow'.
        use std::collections::VecDeque;
        let mut myq = VecDeque::from_iter(
            in_degrees
                .iter()
                .enumerate()
                .filter(|(_, n)| **n == 0)
                .map(|(i, _)| i),
        );
        // println!("myq={myq:?}");
        let mut n_marked = 0;
        while let Some(node) = myq.pop_front() {
            n_marked += 1;
            for nb in adjlist[node].iter() {
                in_degrees[*nb] -= 1;
                if in_degrees[*nb] == 0 {
                    myq.push_back(*nb);
                }
            }
        }
        n_marked == num_courses
    }

    pub fn can_finish_dfs_with_memoization(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // detect cycle
        fn dfs(
            adjlist: &[Vec<usize>],
            node: usize,
            no_cycle: &mut [bool],
            visited: &mut [bool],
        ) -> bool {
            if visited[node] {
                // cycle detected
                return true;
            }

            if no_cycle[node] {
                // impossible to form a cycle starting from this node
                return false;
            }

            visited[node] = true;

            for neibor in adjlist[node].iter() {
                if dfs(adjlist, *neibor, no_cycle, visited) {
                    return true;
                }
            }

            visited[node] = false;
            no_cycle[node] = true;

            false
        }

        let num_courses = num_courses as usize;
        let mut adjlist = vec![vec![]; num_courses];
        for v in prerequisites {
            adjlist[v[1] as usize].push(v[0] as usize);
        }

        // println!("adjlist={adjlist:?}");

        // each element denotes that node can not be possible to form a cycle
        let mut no_cycle = vec![false; num_courses];
        for i in 0..num_courses {
            let mut visited = vec![false; num_courses];

            // println!("node={i}, seen={}", no_cycle[i]);
            if !no_cycle[i] && dfs(&adjlist[..], i, &mut no_cycle, &mut visited) {
                return false;
            }
        }
        true
    }

    // Time Limit Exceeded
    // see test_example_3 and the [link](https://excalidraw.com/#json=Kwa07DRmyZsXh5byjVdff,W8fBgrkGzYu6MenxbpqQJQ)
    pub fn can_finish_dfs_without_memoization(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> bool {
        // detect cycle
        fn dfs(adjlist: &[Vec<i32>], node: i32, seen: &mut [bool], visited: &mut [bool]) -> bool {
            if visited[node as usize] {
                return true;
            }

            seen[node as usize] = true;
            visited[node as usize] = true;

            for neibor in adjlist[node as usize].iter() {
                if dfs(adjlist, *neibor, seen, visited) {
                    return true;
                }
            }

            visited[node as usize] = false;

            false
        }

        let mut adjlist = vec![vec![]; num_courses as usize];
        for v in prerequisites {
            adjlist[v[1] as usize].push(v[0]);
        }

        // println!("adjlist={adjlist:?}");

        let mut seen = vec![false; num_courses as usize];
        for i in 0..num_courses {
            let mut visited = vec![false; num_courses as usize];

            // println!("node={i}, seen={}", seen[i as usize]);
            if !seen[i as usize] && dfs(&adjlist, i, &mut seen, &mut visited) {
                return false;
            }
        }
        true
    }
}
