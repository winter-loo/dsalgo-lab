pub struct Solution;

impl Solution {
    //  s=ababcbacadefegdehijhklij
    //
    //  First, let's understand the problem in depth.
    //  Say we partition s at index 1 such that p1=a p2=babc...
    //  Is this partition OK?
    //  We can find 'a' in both p1 and p2, so it's an invalid partition.
    //
    //  i │ 0  1  2  3  4  5  6  7  8  9  10 11 12  13  14  15  16  17  18  19  20  21  22  23
    //  x │ a  b  a  b  c  b  a  c  a  d  e  f  e   g   d   e   h   i   j   h   k   l   i   j
    //
    //      index       partitions
    //       1          [a] [babac...]      ✗
    //       2          [ab] [ab...]        ✗
    //       3          [aba] [bcb...]      ✗
    //       ..
    //
    // OK! We are getting the feel.
    //
    // for i=0 x=a, for a valid partition, we need find from tail the last
    // character 'a', which is at the index 8.
    //
    // for i=9 x=d, apply the same principle, we find the last index of 'd' at 14.
    // Hence, this partition at least includes s[9..14]
    //
    //                                 j                k
    //                                 ↓                ↓
    //  i │ 0  1  2  3  4  5  6  7  8  9  10 11 12  13  14  15  16  17  18  19  20  21  22  23
    //  x │ a  b  a  b  c  b  a  c  a  d  e  f  e   g   d   e   h   i   j   h   k   l   i   j
    //
    //  At this moment, we might need to expand the right boundary.
    //  So, we check characters in the (j,k).
    //  Found 'e' beyond the 'k'.
    //
    //                                 j                    k
    //                                 ↓                    ↓
    //  i │ 0  1  2  3  4  5  6  7  8  9  10 11 12  13  14  15  16  17  18  19  20  21  22  23
    //  x │ a  b  a  b  c  b  a  c  a  d  e  f  e   g   d   e   h   i   j   h   k   l   i   j
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashSet;

        let mut j = 0;
        let s: Vec<_> = s.chars().collect();
        let mut ps = vec![];

        while j < s.len() {
            let k = s[j+1..].iter().rposition(|c| *c == s[j]).unwrap_or(s.len());
            // println!("first, j={j} k={k}");
            // not found
            if k == s.len() {
                ps.push(1i32);
                j += 1;
                continue;
            }
            let mut k = j + 1 + k;
            let mut seen = HashSet::new();
            // exclude the boundaries
            let mut i = j + 1;
            while i < k {
                if seen.contains(&s[i]) {
                    i += 1;
                    continue;
                }
                seen.insert(s[i]);
                if let Some(n) = s[k+1..].iter().rposition(|c| *c == s[i]) {
                    // println!("second, k={} n={}", k, n);
                    // found the char beyond the boundary
                    // update k to the found position
                    k = k + 1 + n;
                }
                i += 1;
            }
            ps.push((k - j + 1) as i32);
            j = k + 1;
        }

        ps
    }
}
