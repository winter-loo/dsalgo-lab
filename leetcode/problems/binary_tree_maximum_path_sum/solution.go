type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func maxPathSum(root *TreeNode) int {
    a, b := dfs(root)
    return max(a, b)
}

// dfs returns a maximum path sum of between left and right path and
// the path sum of root.val + left path sum + right path sum
//
// Key base idea: think from small to big. If you can solve the 
// small problem, you can easily solve the big problem
func dfs(root *TreeNode) (int, int) {
    if root == nil {
        panic("root should not be null")
    }
    if root.Left == nil && root.Right == nil {
        return root.Val, root.Val
    }
    // -1000 is the minimum input value per the problem description
    m1, m2 := -1001, -1001
    s := 0
    if root.Left != nil {
        left_sum, m := dfs(root.Left)
        m1 = max(m1, left_sum)
        m2 = max(m2, m)
        s += left_sum
    }
    if root.Right != nil {
        right_sum, m := dfs(root.Right)
        m1 = max(m1, right_sum)
        m2 = max(m2, m)
        s += right_sum
    }
    m1 = max(root.Val, root.Val + m1)
    m2 = max(m1, m2, root.Val + s)
    return m1, m2
}

func max(nums ...int) int {
    if len(nums) == 0 {
        panic("max() requires at least one argument")
    }
    maxVal := nums[0]
    for _, v := range nums[1:] {
        if v > maxVal {
            maxVal = v
        }
    }
    return maxVal
}
