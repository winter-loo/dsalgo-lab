package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
  var levelNodes [][]int
  myimpl(root, 0, &levelNodes)
  var rviews []int
  for i := 0; i < len(levelNodes); i++ {
    rviews = append(rviews, levelNodes[i][len(levelNodes[i]) - 1])
  }
  return rviews
}

func myimpl(root *TreeNode, level int, levelNodes *[][]int) {
  if root == nil {
    return
  }
  // fmt.Println("root", root.Val, "level", level, "levelNodes", levelNodes)
  if len(*levelNodes) < level + 1 {
    *levelNodes = append(*levelNodes, []int{root.Val})
  } else {
    // (*levelNodes)[level] = append((*levelNodes)[level], root.Val)
    (*levelNodes)[level][0] = root.Val
  }
  // fmt.Println("root", root.Val, "level", level, "new levelNodes", levelNodes)

  myimpl(root.Left, level + 1, levelNodes)
  myimpl(root.Right, level + 1, levelNodes)
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
	values := []int{1, 2, 3, -1, 5, -1, 4}
	root := buildTree(values, 0)
	printNode(root, " ", false)

	rview := rightSideView(root)
	fmt.Println(rview)
	expected := []int{1, 3, 4}
	for i := 0; i < len(expected) && i < len(rview); i++ {
		if expected[i] != rview[i] {
			fmt.Println("failed")
		}
	}

	values = []int{1, 2, 3, 4, -1, -1, -1, 5}
	root = buildTree(values, 0)
	printNode(root, " ", false)

	rview = rightSideView(root)
	fmt.Println(rview)
	expected = []int{1, 3, 4, 5}
	for i := 0; i < len(expected) && i < len(rview); i++ {
		if expected[i] != rview[i] {
			fmt.Println("failed")
		}
	}


	values = []int{1, -1, 3}
	root = buildTree(values, 0)
	printNode(root, " ", false)

	rview = rightSideView(root)
	fmt.Println(rview)
	expected = []int{1, 3}
	for i := 0; i < len(expected) && i < len(rview); i++ {
		if expected[i] != rview[i] {
			fmt.Println("failed")
		}
	}


	values = []int{}
	root = buildTree(values, 0)
	printNode(root, " ", false)

	rview = rightSideView(root)
	fmt.Println(rview)
	expected = []int{}
	for i := 0; i < len(expected) && i < len(rview); i++ {
		if expected[i] != rview[i] {
			fmt.Println("failed")
		}
	}
}
