pub struct Solution;

impl Solution {
    // Input: nums = [3,1,5,8]
    // Still a decision/selection problem.
    // Initially, you have 4 choices, then 3, then 2, then 1.
    // Hence, we can use decision tree to solve this problem.
    //
    //  Decision tree here:
    // 'https://excalidraw.com/#json=mDsKWddH5cPdlebx29tQ-,qpZEGZW0Kj6EXOOh9YeMAQ'
    //
    // From the above tree, we can get some conclusions:
    // - when there is only 1 balloon, the max coins you can get is the number in it.
    // - when there are 2 balloons, always choose the smaller one first
    // - when there are 3 balloons, ...
    // - when there are 4 balloons, ...
    //
    //              3       1       5       8
    //            ┌───────────────────────────
    //        3 1 │ 3       1       5       8
    //        1 2 │
    //        5 3 │
    //        8 4 │
    //
    //  It's hard to think like that. Maybe we need try other methods.
    //
    //  ───────────────────────────────────────────────────────────────────
    //
    //  In previous logic, we try to go from one end to the other end. We could
    //  switch logic, how about from the middle to both sides.
    //
    //  3 (1 5) 8 ┌─choose 1──> 3 (5) 8, we got 15 ──choose 5──> 3 8, we got 135
    //            └─choose 5──> 3 (1) 8, we got 50 ──choose 1──> 3 8, we got 74
    //
    //  Pattern = A (M) B
    //  M can contain many numbers and will aggregate into only one number.
    //  Hence, M is the subproblem.
    //  Say, A is '3', M is '1 5 8', B is '8'
    //  For M='0 5 8', we need always select 0 first.
    //
    //  ───────────────────────────────────────────────────────────────────
    //
    //  3 1 5 8, how about always choose the smallest first?
    //
    //  1 -> 3 5 8, 15
    //  3 -> 5 8, 15+15=30
    //  5 -> 8, 30+40=70
    //  8 -> , 70+8=78
    //
    //  Not the best solution. Hence, greedy algorithm is not working.
    //
    //  ───────────────────────────────────────────────────────────────────
    //
    //  Consider the following:
    //  - if the sides have zero, remove the zero first
    //  - otherwise, we always choose the smallest in the middle
    //
    // For 3 1 5 8, no zero on both sides, so
    //
    // 1 -> 3 5 8, 15
    // 5 -> 3 8, 120 + 15 = 135
    // 3 -> 8, 24 + 135 = 159
    // 8 -> , 8 + 159 = 167
    //
    // Feasible!
    //
    // For 2 3 5,
    // 3 -> 2 5, 30
    // 2 -> 5, 10 + 30 = 40
    // 5 -> , 5 + 40 = 45
    //
    // For implementation, need an algorithm choose the smallest among a list of
    // numbers. The hard part is: when we know the index of the smallest number,
    // we need to get its neighbors indicies.
    //
    // For '3 1 5 8', the smallest number in the middle is 1 and its index is 1
    // and its neighbors indices are 0 and 2. The next smallest number in the
    // middle is 5 and its index is 2 and its original neighbors indices are
    // 1 and 3. However, at this time, the number at the index 1 is supposed to
    // be removed. Hence, the neighbors of the index 2 should be indices of 0
    // and 2.
    //
    // 2,(1,3) -> 2,(0,3)
    //
    // For simplicity, we maybe need maintain each element's neighbors indices.
    //
    // for 1, its neighbors are (0,2)
    // for 2, its neighbors are (1,3)
    // when 1 gets removed, its neighbors (0,2) need update its neighbors. 0 is
    // not in considering, so the index 2 needs update its neighbors to (0,3).
    //
    // Hey, use linked list for this kind of thing!
    //
    // For a more complex example, say, '3 1 7 5 8', the M='1 7 5' and the initial
    // neighbor chain is:
    //
    //      0 -> 1 -> 2 -> 3 -> 4
    //      3    1    7    5    8
    //
    // when 1 gets removed, the chain is changed into:
    //
    //      0 -> 2 -> 3 -> 4
    //      3    7    5    8
    //
    //  ──────────────────────wrong algorithm ───────────────────────────
    //
    //  for '1 3 4 5', the result is wrong.
    pub fn max_coins_wrong(nums: Vec<i32>) -> i32 {
        // remove zeros from both sides first
        let nums = nums.strip_prefix(&[0]).unwrap_or(&nums[..]);
        let nums = nums.strip_suffix(&[0]).unwrap_or(nums);

        // build an ordered map, with key is the number and value is the array index
        use std::collections::BTreeMap;
        let mut ordered = BTreeMap::new();

        if nums.len() == 1 {
            return nums[0];
        }

        println!("nums={:?}", &nums[1..nums.len() - 1]);
        for (i, n) in nums[1..nums.len() - 1].iter().enumerate() {
            println!("i={} n={n}", i);
            ordered.entry(n).or_insert(vec![]).push(i + 1);
        }

        let mut neighbor_chain = ChainList::from(nums);

        let mut sum = 0;
        for (n, indices) in ordered.iter() {
            for i in indices {
                let nber = neighbor_chain.get_neighbors(*i);
                let prev = nber.0.unwrap();
                let next = nber.1.unwrap();
                println!("n={} i={}", next, prev);
                sum += dbg!(dbg!(*n) * dbg!(nums[prev]) * dbg!(nums[next]));
                neighbor_chain.remove(*i);
            }
        }
        println!("add both sides");
        sum += nums[0] * nums[nums.len() - 1] + nums[nums.len() - 1];
        sum
    }

    // 1 3 4 5
    // 1 * 1 * 3, 3 4 5 => 3 + 80 = 83
    // 1 * 3 * 4, 1 4 5 => 12 + 30 = 42
    // 3 * 4 * 5, 1 3 5 => 60 + 25 = 85 ✅
    // 4 * 5 * 1, 1 3 4 => 20 + 20 = 40
    //
    // 3 4 5
    // 1 * 3 * 4, 4 5 => 12 + 25 = 37
    // 3 * 4 * 5, 3 5 => 60 + 20 = 80 ✅
    // 4 * 5 * 1, 3 4 => 20 + 16 = 36
    //
    // 1 4 5
    // 1 * 1 * 4, 4 5 => 4 + 25 = 29
    // 1 * 4 * 5, 1 5 => 20 + 10 = 30 ✅
    // 4 * 5 * 1, 1 4 => 20 + 8 = 28
    //
    // 1 3 5
    // 1 * 1 * 3, 3 5 => 3 + 20 = 23
    // 1 * 3 * 5, 1 5 => 15 + 10 = 25 ✅
    // 3 * 5 * 1, 1 3 => 15 + 6 = 21
    //
    // 1 3 4
    // 1 * 1 * 3, 3 4 => 3 + 14 = 17
    // 1 * 3 * 4, 1 4 => 12 + 8 = 20 ✅
    // 3 * 4 * 1, 1 3 => 12 + 6 = 18
    //
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        fn recur(nums: &[i32], neighbor_chain: ChainList, sum: &mut i32, max_sum: &mut i32) {
            if neighbor_chain.empty() {
                max_sum = max_sum.max(sum);
                return;
            }
            for (i, n) in neighbor_chain.iter().enumerate() {
                let nber = neighbor_chain.get_neighbors(*i);
                let prev = nber.0.unwrap();
                let next = nber.1.unwrap();
                sum += nums[i] * nums[prev] * nums[next];
                println!("prev={prev} next={next}");
                recur(nums, neighbor_chain, sum, max_sum);
            }
        }
        let mut max_sum = 0;
        let mut sum = 0;
        recur(&nums[..], neighbor_chin, &mut sum, &mut max_sum)
        max_sum
    }
}

#[derive(Debug, Clone)]
struct Link {
    prev: i32,
    next: i32,
    used: bool,
}

impl Link {
    fn new() -> Link {
        Link {
            prev: -1,
            next: i32::MAX,
            used: false,
        }
    }
}

struct ChainList {
    nodes: Vec<Link>,
    initial: usize,
    len: usize,
}

impl ChainList {
    fn from(nums: &[i32]) -> Self {
        let mut nodes = vec![Link::new(); nums.len()];
        for (i, node) in nodes.iter_mut().enumerate() {
            node.prev = i as i32 - 1;
            node.next = i as i32 + 1;

            if i == nums.len() - 1 {
                node.next = i32::MAX;
            }
        }
        ChainList { nodes, initial: 0, len: nums.len(), }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn remove(&mut self, idx: usize) {
        let (prev, next) = if let Some(node) = self.nodes.get_mut(idx) {
            node.used = true;
            (node.prev, node.next)
        } else {
            (-1, i32::MAX)
        };

        if prev >= 0 {
            self.nodes[prev as usize].next = next;
        }
        if next != i32::MAX {
            self.nodes[next as usize].prev = prev;
        }
    }

    fn get_neighbors(&self, idx: usize) -> (Option<usize>, Option<usize>) {
        (
            if self.nodes[idx].prev < 0 {
                None
            } else {
                Some(self.nodes[idx].prev as usize)
            },
            if self.nodes[idx].next == i32::MAX {
                None
            } else {
                Some(self.nodes[idx].next as usize)
            },
        )
    }
}
