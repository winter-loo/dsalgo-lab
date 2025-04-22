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
func dfs(root *TreeNode) (int, int) {
    if root == nil {
        panic("root should not be null")
    }
    if root.Left == nil && root.Right == nil {
        return root.Val, root.Val
    }
    left_sum, right_sum, m1, m2 := 0, 0, 0, 0
    if root.Left != nil {
        left_sum, m1 = dfs(root.Left)
    }
    if root.Right != nil {
        right_sum, m2 = dfs(root.Right)
    }
    return root.Val + max(left_sum, right_sum), max(m1, m2, root.Val + left_sum + right_sum)
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
