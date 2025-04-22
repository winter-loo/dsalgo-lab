use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap = BinaryHeap::new();
        let k = k as usize;

        for point in points {
            let dist = (point[0] * point[0] + point[1] * point[1]) as i64;
            // Push distance and point reference (or clone)
            max_heap.push((dist, point));
            // If heap size exceeds k, remove the farthest point
            if max_heap.len() > k {
                max_heap.pop();
            }
        }

        // Extract points from the heap
        max_heap.into_iter().map(|(_, point)| point).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn vec_to_set(vec: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        vec.into_iter().collect()
    }

    #[test]
    fn test_k_closest_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let expected = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, k), expected);
    }

    #[test]
    fn test_k_closest_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        // Distances squared: (3*3+3*3)=18, (5*5+(-1)*(-1))=26, (-2)*(-2)+4*4)=20
        // Closest 2 are [3, 3] and [-2, 4]
        let expected = vec![vec![3, 3], vec![-2, 4]];
        // Order doesn't matter, so compare sets
        let result = Solution::k_closest(points, k);
        assert_eq!(vec_to_set(result), vec_to_set(expected));
    }

     #[test]
    fn test_k_closest_duplicates() {
        let points = vec![vec![0,1],vec![1,0]];
        let k = 2;
        let expected = vec![vec![0,1],vec![1,0]];
        let result = Solution::k_closest(points, k);
        assert_eq!(vec_to_set(result), vec_to_set(expected));
    }

    #[test]
    fn test_k_closest_large_k() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let k = 3;
        let expected = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = Solution::k_closest(points, k);
        assert_eq!(vec_to_set(result), vec_to_set(expected));
    }
}
