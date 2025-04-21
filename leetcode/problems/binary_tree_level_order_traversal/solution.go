package main

import "fmt"

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

// levelOrder performs a level-order traversal (BFS) on the binary tree.
// It initializes the result slice and calls a recursive helper function to populate it.
func levelOrder(root *TreeNode) [][]int {
    // level tracks the current depth during traversal (starts at 0 for the root).
    // levelWise stores the node values grouped by level.
    level, levelWise := 0, [][]int{}
    // Call the recursive helper function to perform the traversal.
    levelTraversal(root, &level, &levelWise)
    // Return the populated slice containing nodes level by level.
    return levelWise
}

// levelTraversal is a recursive helper function (using DFS approach) 
// to populate the levelWise slice based on node depth.
func levelTraversal(root *TreeNode, level *int, levelWise *[][]int) {
    // Base case: If the current node is nil, stop recursion for this branch.
    if root == nil { return }

    // Check if the slice for the current level needs to be created.
    if len(*levelWise) < *level + 1 {
        // If the current level doesn't exist in levelWise yet, create it
        // by appending a new slice containing the current node's value.
        *levelWise = append(*levelWise, []int{root.Val})
    } else {
        // If the slice for the current level already exists, append the
        // current node's value to that slice.
        (*levelWise)[*level] = append((*levelWise)[*level], root.Val)
    }

    // Increment level before visiting children.
    *level += 1
    // Recursively traverse the left subtree.
    levelTraversal(root.Left, level, levelWise)
    // Recursively traverse the right subtree.
    levelTraversal(root.Right, level, levelWise)
    // Decrement level after returning from children (backtracking step).
    *level -= 1
}

func buildTree(values []int, index int) *TreeNode {
    if index >= len(values) || values[index] == -1 {
        return nil
    }
    
    node := &TreeNode{Val: values[index]}
    node.Left = buildTree(values, 2*index+1)
    node.Right = buildTree(values, 2*index+2)
    
    return node
}

func printNode(node *TreeNode) {
    if node == nil {
        fmt.Printf("%s,", "nil")
        return;
    }
    fmt.Printf("%d,", node.Val)
    printNode(node.Left)
    printNode(node.Right)
    fmt.Println()
}

func main() {
    values := []int{6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5}
    root := buildTree(values, 0)
    printNode(root)
    levelOrder(root)
}
