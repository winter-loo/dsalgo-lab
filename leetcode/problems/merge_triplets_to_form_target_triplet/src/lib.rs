pub struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        Solution::merge_triplets_greedy(triplets, target)
    }

    // [[2,5,3],[1,8,4],[1,7,5]] T=[2 7 5]
    //
    // First, find the elements such that element[0] <= target[0]
    // Then, find the elements such that element[1] <= target[1] where all elements
    // satisfy the condition in the first step.
    // Finally, find the element such that element[2] == target[2]
    //
    // ──────────────────────wrong above──────────────────────────────────────
    //
    // Remember, we need update the other. This action causes a side effect for
    // subsequent comparison.
    //
    // Hence, think back. This is a combinatorial problem. We need select 2 out
    // of 3 first. Each choice produces a new result.
    // The bad news is that we can select any two elements such that the remaining
    // elements are not in sequential order.
    //
    // [2 5 3] [1 8 4] ──▶ [2 5 3] [2 8 4] or [2 8 4] [1 8 4]
    //
    // [2 8 4] [1 7 5] ──▶ [2 8 4] [2 8 5] or [1 7 5] [2 8 5]
    //
    //
    // [2 5 3] [1 7 5] ──▶ [2 7 5] [2 5 3] or [2 7 5] [ 1 7 5] ✓
    //
    // Here comes the brute force solution
    pub fn merge_triplets_brute_force(mut triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
            vec![a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
        }
        fn inner(triplets: &mut Vec<Vec<i32>>, target: &Vec<i32>) -> bool {
            if triplets.len() == 1 {
                return triplets[0] == *target;
            }
            for i in 0..triplets.len() {
                for j in i + 1..triplets.len() {
                    let merged = merge(&triplets[i], &triplets[j]);
                    if merged == *target {
                        return true;
                    }
                    if merged == triplets[i] || merged == triplets[j] {
                        continue;
                    }
                    let backup = triplets[j].clone();
                    triplets[j] = merged;
                    if inner(triplets, target) {
                        return true;
                    }
                    triplets[j] = backup;
                }
            }
            false
        }
        inner(&mut triplets, &target)
    }

    // [[2,5,3],[1,8,4],[1,7,5]] T=[2 7 5]
    // find [2 5 3]
    // [2 5 3] [1 8 4] --> [2 8 4]
    // [2 5 3] [1 7 5] --> [2 7 5] ✓
    //
    // [3 1 7] [1 5 10] T=[1 3 1]
    pub fn merge_triplets_brute_force2(mut triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
            vec![a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
        }
        fn inner(triplets: &mut Vec<Vec<i32>>, target: &Vec<i32>, n: usize) -> bool {
            if triplets.len() == 1 {
                return triplets[0] == *target;
            }
            if n == 3 {
                return false;
            }
            let mut candidates = Vec::with_capacity(triplets.len());
            let mut candidates2 = Vec::with_capacity(triplets.len());
            for (i, tri) in triplets.iter().enumerate() {
                if tri[n] == target[n] {
                    candidates.push(i);
                } else if tri[n] < target[n] {
                    candidates2.push(i);
                }
            }
            for i in candidates {
                for j in &candidates2 {
                    let j = *j;
                    // exclude itself
                    if i == j {
                        continue;
                    }
                    let merged = merge(&triplets[i], &triplets[j]);
                    if merged == *target {
                        return true;
                    }
                    // exclude already compared
                    if merged == triplets[i] || merged == triplets[j] {
                        continue;
                    }
                    let backup = triplets[j].clone();
                    triplets[j] = merged;
                    if inner(triplets, target, n + 1) {
                        return true;
                    }
                    triplets[j] = backup;
                }
            }
            false
        }
        inner(&mut triplets, &target, 0)
    }

    // [[2,5,3],[1,8,5],[1,7,4]] T=[2 7 5]
    //
    // found 2
    //
    // found 7
    //
    // merge: [2 5 3] and [1 7 4]
    // [2 7 4] [2 5 3] [1 8 5] or [2 7 4] [1 7 4] [1 8 5]
    //
    // for [2 7 4] [2 5 3] [1 8 5]
    // found 5
    // merge [2 7 4] [1 8 5] -> [2 8 5] ✗
    //
    // for [2 7 4] [1 7 4] [1 8 5]
    // found 5
    // merge [2 7 4] [1 8 5] -> [2 8 5] ✗
    //
    // Accepted! However, beats 0% on CPU and memory
    pub fn merge_triplets_greedy(mut triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
            vec![a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
        }
        if triplets.len() == 1 {
            return triplets[0] == target;
        }

        // sort beforhand so as to avoid full scan in **location 2**
        triplets.sort_by(|v1, v2| v1[0].cmp(&v2[0]));

        // let mut candi1 = Vec::with_capacity(triplets.len());
        // let mut candi2 = Vec::with_capacity(triplets.len());
        let mut candi1 = vec![];
        let mut candi2 = vec![];
        for (i, tri) in triplets.iter().enumerate() {
            if tri[0] == target[0] {
                candi1.push(i);
            }
            if tri[1] == target[1] {
                candi2.push(i);
            }
        }
        if candi1.is_empty() || candi2.is_empty() {
            return false;
        }

        for i in candi1 {
            for j in &candi2 {
                let j = *j;
                // as the triplets already sorted by the first element in ascending
                // order, we could know only element at the left hand side of this
                // element will satisfy the condition: triplets[j][0] <= triplets[i][0]
                if j > i {
                    continue;
                }
                let merged = merge(&triplets[i], &triplets[j]);
                if merged == target {
                    return true;
                }
                // **location 2**
                for tri in triplets[0..=i].iter() {
                   if tri[2] == target[2] && merge(&merged, tri) == target {
                       return true;
                   }
                }
            }
        }
        false
    }
}
