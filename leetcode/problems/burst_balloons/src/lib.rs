pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        Self::max_coins_memoization(nums)
    }

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

        // println!("nums={:?}", &nums[1..nums.len() - 1]);
        for (i, n) in nums[1..nums.len() - 1].iter().enumerate() {
            // println!("i={} n={n}", i);
            ordered.entry(n).or_insert(vec![]).push(i + 1);
        }

        let mut neighbor_chain = ChainList::from(nums);

        let mut sum = 0;
        for (n, indices) in ordered.iter() {
            for i in indices {
                let nber = neighbor_chain.get_neighbors(*i);
                let prev = nber.0.unwrap();
                let next = nber.1.unwrap();
                // println!("n={} i={}", next, prev);
                sum += dbg!(dbg!(*n) * dbg!(nums[prev]) * dbg!(nums[next]));
                neighbor_chain.remove(*i);
            }
        }
        // println!("add both sides");
        sum += nums[0] * nums[nums.len() - 1] + nums[nums.len() - 1];
        sum
    }

    // Back to our first thought. Recursively walking the decision tree.
    //
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
    // Correct now but Time Limit Exceeded
    pub fn max_coins_brute_force(nums: Vec<i32>) -> i32 {
        fn recur(nums: &[i32], neighbor_chain: &ChainList, sum: &mut i32, max_sum: &mut i32) {
            // println!("chain={:?}", neighbor_chain);
            if neighbor_chain.is_empty() {
                *max_sum = (*max_sum).max(*sum);
                // println!("──────────path done──────────\n");
                return;
            }
            for link in neighbor_chain.iter() {
                let prev_value = if let Some(prev) = link.prev() {
                    nums[prev]
                } else {
                    1
                };
                let next_value = if let Some(next) = link.next() {
                    nums[next]
                } else {
                    1
                };
                // println!(
                //   "current={} prev={prev_value} next={next_value}",
                //  nums[link.curr]
                // );
                let mut nc = neighbor_chain.clone();
                nc.remove(link.curr);
                let value = nums[link.curr] * prev_value * next_value;
                *sum += value;
                recur(nums, &nc, sum, max_sum);
                *sum -= value;
            }
        }
        let mut max_sum = 0;
        let mut sum = 0;
        let neighbor_chain = ChainList::from(&nums);
        recur(&nums[..], &neighbor_chain, &mut sum, &mut max_sum);
        max_sum
    }

    // D=[1,2,3,4,5]
    //
    // f(D,[]) = Max(
    //   1 * 1 * 2 + f(D-1),
    //   1 * 2 * 3 + f(D-2),
    //   2 * 3 * 4 + f(D-3),
    //   3 * 4 * 5 + f(D-4),
    //   4 * 5 * 1 + f(D-5)
    // )
    //
    // f(D-1) = f(2 3 4 5)
    // f(D-2) = f(1 3 4 5)
    // f(D-3) = f(1 2 4 5)
    // f(D-4) = f(1 2 3 5)
    // f(D-5) = f(1 2 3 4)
    //
    // f(2 3 4 5) and f(1 3 4 5) have the same subproblem f(3 4 5)
    // f(1 2 3 5) and f(1 2 3 4) have the same subproblem f(1 2 3)
    //
    // f(2 3 4 5)  ─┐
    // f(1 3 4 5)   ├─▶ have the same subproblem f(4 5)
    // f(1 2 4 5)  ─┘
    //
    // f(1 2 4 5)  ─┐
    // f(1 2 3 5)   ├─▶ have the same subproblem f(1 2)
    // f(1 2 3 4)  ─┘
    //
    // f(4 5) = 4 * 5 + 5 = 25
    // f(1 2) = 1 * 2 + 2 = 4
    //
    // f(3 4 5) = Max(
    //   1 * 3 * 4 + f(4 5),
    //   3 * 4 * 5 + f(3 5),
    //   4 * 5 * 1 + f(3 4)
    // )
    //
    // f(2 3 4 5) = Max(
    //   1 * 2 * 3 + f(3 4 5),
    //   2 * 3 * 4 + f(2 4 5),
    //   3 * 4 * 5 + f(2 3 5),
    //   4 * 5 * 1 + f(2 3 4)
    // )
    //
    // f(1 3 4 5) = Max(
    //   1 * 1 * 3 + f(3 4 5),
    //   1 * 3 * 4 + f(1 4 5),
    //   3 * 4 * 5 + f(1 3 5),
    //   4 * 5 * 1 + f(1 3 4)
    // )
    //
    // f(1 2 4 5) = Max(
    //   1 * 1 * 2 + f(2 4 5),
    //   1 * 2 * 4 + f(1 4 5),
    //   2 * 4 * 5 + f(1 2 5),
    //   4 * 5 * 1 + f(1 2 4)
    // )
    //
    // From the above computations, we could see we need repeatly computing:
    //
    // f(3 4 5), f(2 4 5), f(1 4 5)
    //
    // How could we encode these states out of [1 2 3 4 5]?
    //
    // how about using bitmap?
    // We use 5 bits to encode [1 2 3 4 5]. For the whole array, it could be encoded
    // as: 0b11111 = 0x1f
    //
    // [3 4 5] ──encode──▶ 0b00111 = 0x07
    // [2 4 5] ──encode──▶ 0x01011 = 0x0b
    // [1 4 5] ──encode──▶ 0x10011 = 0x13
    //
    // For this problem, we need at most 300 bits, i.e, 38 bytes.
    //
    // Hence, use Bitmap as the key to lookup memoization table.
    //
    // struct Bitmap {
    //   bits: [u8; 38]
    // }
    pub fn max_coins_memoization(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn recur(
            nums: &[i32],
            neighbor_chain: &ChainList,
            max_sum: &mut i32,
            mem: &mut HashMap<Bitmap, i32>,
        ) -> i32 {
            // println!("chain={:?}, bitmap={:?}", neighbor_chain, neighbor_chain.bitmap());
            if neighbor_chain.is_empty() {
                // println!("────────────────────return 0────────────────────\n");
                return 0;
            }
            if let Some(sum) = mem.get(&neighbor_chain.bitmap()) {
                // println!("────────────────────return sum={sum} from mem────────────────────\n");
                return *sum;
            }
            let mut local_max_sum = 0;
            for link in neighbor_chain.iter() {
                let prev_value = if let Some(prev) = link.prev() {
                    nums[prev]
                } else {
                    1
                };
                let next_value = if let Some(next) = link.next() {
                    nums[next]
                } else {
                    1
                };
                // println!(
                //     "current={} prev={prev_value} next={next_value}",
                //     nums[link.curr]
                // );
                let mut nc = neighbor_chain.clone();
                nc.remove(link.curr);
                let gain = nums[link.curr] * prev_value * next_value;
                let value = recur(nums, &nc, max_sum, mem);
                local_max_sum = local_max_sum.max(gain + value);
            }
            *max_sum = (*max_sum).max(local_max_sum);
            mem.insert(neighbor_chain.bitmap(), local_max_sum);
            // println!("────────────────────return local_max_sum={local_max_sum}────────────────────\n");
            local_max_sum
        }
        let mut max_sum = 0;
        let neighbor_chain = ChainList::from(&nums);
        let mut mem = HashMap::new();
        recur(&nums[..], &neighbor_chain, &mut max_sum, &mut mem);
        max_sum
    }
}

#[derive(Debug, Clone, Copy)]
struct Link {
    curr: usize,
    prev: i32,
    next: i32,
    used: bool,
}

impl Link {
    fn new() -> Link {
        Link {
            curr: usize::MAX,
            prev: -1,
            next: i32::MAX,
            used: false,
        }
    }

    fn next(&self) -> Option<usize> {
        if self.next == i32::MAX {
            None
        } else {
            Some(self.next as usize)
        }
    }

    fn prev(&self) -> Option<usize> {
        if self.prev < 0 {
            None
        } else {
            Some(self.prev as usize)
        }
    }
}

#[derive(Clone)]
struct ChainList {
    links: Vec<Link>,
    initial: i32,
    len: usize,
}

impl ChainList {
    fn from(nums: &[i32]) -> Self {
        // println!("construct chain from nums={nums:?}");
        let mut nodes = vec![Link::new(); nums.len()];
        for (i, node) in nodes.iter_mut().enumerate() {
            node.curr = i;
            node.prev = i as i32 - 1;
            node.next = i as i32 + 1;

            if i == nums.len() - 1 {
                node.next = i32::MAX;
            }
        }
        ChainList {
            links: nodes,
            initial: 0,
            len: nums.len(),
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn remove(&mut self, idx: usize) {
        let (prev, next) = if let Some(node) = self.links.get_mut(idx) {
            node.used = true;
            self.len -= 1;
            if idx as i32 == self.initial {
                self.initial = node.next;
            }
            (node.prev, node.next)
        } else {
            (-1, i32::MAX)
        };

        if prev >= 0 {
            self.links[prev as usize].next = next;
        }
        if next != i32::MAX {
            self.links[next as usize].prev = prev;
        }
    }

    fn get_neighbors(&self, idx: usize) -> (Option<usize>, Option<usize>) {
        (
            if self.links[idx].prev < 0 {
                None
            } else {
                Some(self.links[idx].prev as usize)
            },
            if self.links[idx].next == i32::MAX {
                None
            } else {
                Some(self.links[idx].next as usize)
            },
        )
    }

    fn iter(&self) -> ChainListIter {
        ChainListIter {
            list: self,
            current: self.initial,
        }
    }

    fn bitmap(&self) -> Bitmap {
        let mut bitmap = Bitmap::new();
        for link in self.iter() {
            bitmap.add(link.curr);
        }
        bitmap
    }
}

const MAX_BYTES_FOR_BITMAP: usize = 38 + 2;
const BITMAP_LEN: usize = MAX_BYTES_FOR_BITMAP / 8;

#[derive(Eq, PartialEq, Hash)]
struct Bitmap([u64; BITMAP_LEN]);

impl Bitmap {
    fn new() -> Self {
        Bitmap([0; BITMAP_LEN])
    }

    fn add(&mut self, num: usize) {
        let bits = &mut self.0;
        let seg = bits.get_mut(num / 64).unwrap();
        *seg |= 1 << (num % 64);
    }
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.0.iter().enumerate() {
            write!(f, "{:b}", n)?;
            if i != self.0.len() - 1 {
                write!(f, "_")?;
            }
        }
        write!(f, "")
    }
}

impl std::fmt::Debug for ChainList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")?;
        } else {
            write!(f, "[")?;
            write!(f, "{} → ", self.initial)?;
            for (i, link) in self.iter().enumerate() {
                write!(f, "{}", link.next)?;
                if i != self.len - 1 {
                    write!(f, " → ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "")
    }
}

struct ChainListIter<'a> {
    list: &'a ChainList,
    current: i32,
}

impl Iterator for ChainListIter<'_> {
    type Item = Link;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 0 || self.current == i32::MAX {
            None
        } else {
            let link = self.list.links[self.current as usize];
            self.current = link.next;
            Some(link)
        }
    }
}
