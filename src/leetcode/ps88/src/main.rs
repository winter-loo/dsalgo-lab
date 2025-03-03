// Title: 88. Merge Sorted Array
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = 0;
        while i >= 0 && j >= 0 {
            nums1[(m + n - 1 - k) as usize] = if nums2[j as usize] >= nums1[i as usize] {
                j -= 1;
                nums2[(j + 1) as usize]
            } else {
                i -= 1;
                nums1[(i + 1) as usize]
            };
            k += 1;
        }
        for k in 0..=j {
            nums1[k as usize] = nums2[k as usize];
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}
