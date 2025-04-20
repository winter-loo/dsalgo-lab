package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

// Optimized LCA function for Binary Search Tree
// Takes advantage of BST properties for more efficient searching
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
    // If root is nil, no LCA exists
    if root == nil {
        return nil
    }
    
    // Get the values for easier comparison
    pVal, qVal := p.Val, q.Val
    
    // If both p and q are less than root, LCA is in left subtree
    if pVal < root.Val && qVal < root.Val {
        return lowestCommonAncestor(root.Left, p, q)
    }
    
    // If both p and q are greater than root, LCA is in right subtree
    if pVal > root.Val && qVal > root.Val {
        return lowestCommonAncestor(root.Right, p, q)
    }
    
    // If one is less and one is greater (or one equals root),
    // then the current root is the LCA
    return root
}

// Helper function to find a node with a specific value in the tree
func findNode(root *TreeNode, val int) *TreeNode {
    if root == nil {
        return nil
    }
    
    if root.Val == val {
        return root
    }
    
    // Use BST property for efficient searching
    if val < root.Val {
        return findNode(root.Left, val)
    } else {
        return findNode(root.Right, val)
    }
}

// Function to build a BST from an array representation
func buildTree(values []int, index int) *TreeNode {
    if index >= len(values) || values[index] == -1 {
        return nil
    }
    
    node := &TreeNode{Val: values[index]}
    node.Left = buildTree(values, 2*index+1)
    node.Right = buildTree(values, 2*index+2)
    
    return node
}

// Helper function to print a tree node
func printNode(node *TreeNode) {
    if node == nil {
        fmt.Println("nil")
    } else {
        fmt.Printf("Node with value: %d\n", node.Val)
    }
}

func main() {
    // Example 1: From LeetCode
    values := []int{6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5}
    root := buildTree(values, 0)
    
    // Find actual node references in the tree
    p := findNode(root, 2)
    q := findNode(root, 8)
    
    fmt.Println("Example 1:")
    fmt.Printf("Finding LCA of nodes with values %d and %d\n", p.Val, q.Val)
    lca := lowestCommonAncestor(root, p, q)
    fmt.Printf("LCA: ")
    printNode(lca)
    
    // Example 2: From LeetCode
    p = findNode(root, 2)
    q = findNode(root, 4)
    fmt.Println("\nExample 2:")
    fmt.Printf("Finding LCA of nodes with values %d and %d\n", p.Val, q.Val)
    lca = lowestCommonAncestor(root, p, q)
    fmt.Printf("LCA: ")
    printNode(lca)
}
