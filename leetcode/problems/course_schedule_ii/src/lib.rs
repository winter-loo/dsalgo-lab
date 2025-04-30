pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::find_order_topological_sort(num_courses, prerequisites)
    }

    pub fn find_order_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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
        let mut learning_order = vec![];
        while let Some(node) = myq.pop_front() {
            learning_order.push(node as i32);
            for nb in adjlist[node].iter() {
                in_degrees[*nb] -= 1;
                if in_degrees[*nb] == 0 {
                    myq.push_back(*nb);
                }
            }
        }
        if learning_order.len() == num_courses {
            learning_order
        } else {
            vec![]
        }
    }
}
