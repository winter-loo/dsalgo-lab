package main

import "fmt"

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
    if len(preorder) != len(inorder) || len(preorder) == 0 {
        return nil
    }
    i := 0
    for ; i < len(inorder); i++ {
        if inorder[i] == preorder[0] {
            break;
        }
    }
    // fmt.Println("i", i)
    root := &TreeNode{Val: preorder[0]}
    // fmt.Println("before preorder", preorder[1:i+1], ", inorder", inorder[0:i])
    root.Left = buildTree(preorder[1:i+1], inorder[0:i])
    // fmt.Println("before2 preorder", preorder[i+1:], ", inorder", inorder[i+1:])
    root.Right = buildTree(preorder[i+1:], inorder[i+1:])
    return root
}

func printNode(node *TreeNode, indent string, isRight bool) {
	if node == nil {
		return
	}

	fmt.Printf("%s", indent)
	if isRight {
		fmt.Printf("R---- ")
		indent += "   "
	} else {
		fmt.Printf("L---- ")
		indent += "|  "
	}

	fmt.Printf("%d\n", node.Val)
	printNode(node.Left, indent, false)
	printNode(node.Right, indent, true)
}

func main() {
  root := buildTree([]int{3,9,20,15,7}, []int{9,3,15,20,7})
  printNode(root, " ", false)
}
