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
    //       3          [aba] [bcb...]      x
    //       ..
    //
    // OK! We are getting the feel.
    //
    // for i=0 x=a, for a valid partition, we need find from tail the last
    // character 'a', which is at the index 8.
    //
    // for i=9 x=d, apply the same priciple, we find the last index of 'd' at 14.
    // Hence, this partition at least includes s[9..14]
    pub fn partition_labels(s: String) -> Vec<i32> {
        todo!()
    }
}
