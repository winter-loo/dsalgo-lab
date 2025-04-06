pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1; nums.len()];

    for i in (0..nums.len() - 1).rev() {
        ans[i] = ans[i + 1] * nums[i + 1];
    }

    let mut prod_left = 1;
    for i in 1..nums.len() {
        prod_left = nums[i - 1] * prod_left;
        ans[i] = ans[i] * prod_left;
    }
    ans
}
