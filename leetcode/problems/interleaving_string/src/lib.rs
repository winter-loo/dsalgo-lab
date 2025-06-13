pub struct Solution;

impl Solution {
    // Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
    // f(s3, s1, s2) = true or false?
    // g(s3, i3, s1, i1, i2) = g(s3, i3 - i, s1, i1 - j, s2, i2 - k) if j + k == i and
    //                              s3[i3..i3+j] = s1[i1..i1+j]
    //                              s3[i3+j..i3+j+k] = s2[i2..i2+k]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        todo!()
    }
}
