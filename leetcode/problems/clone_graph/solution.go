package main

import "fmt"

type Node struct {
    Val int
    Neighbors []*Node
}

func (n *Node) String() string {
  if n == nil {
    return "nil"
  }
  result := fmt.Sprintf("Node{Val: %d, Neighbors: [", n.Val)
  visited := make(map[*Node]bool)
  visited[n] = true
  
  var printNodes func(*Node, map[*Node]bool) string
  printNodes = func(node *Node, visited map[*Node]bool) string {
    nodeStr := ""
    for i, neighbor := range node.Neighbors {
      if i > 0 {
        nodeStr += ", "
      }
      if visited[neighbor] {
        nodeStr += fmt.Sprintf("Node{Val: %d}*", neighbor.Val)
      } else {
        visited[neighbor] = true
        nodeStr += fmt.Sprintf("Node{Val: %d, Neighbors: [%s]}", neighbor.Val, printNodes(neighbor, visited))
      }
    }
    return nodeStr
  }
  
  result += printNodes(n, visited) + "]}"
  return result
}


func cloneGraph(node *Node) *Node {
  seen := make(map[*Node]*Node)
  return deepClone(node, seen)
}

func deepClone(node *Node, seen map[*Node]*Node) *Node {
  if node == nil {
    return nil
  }
  if node_clone, ok := seen[node]; ok {
    return node_clone
  }

  node_clone := Node{Val: node.Val}
  seen[node] = &node_clone

  for _, n := range node.Neighbors {
    node_clone.Neighbors = append(node_clone.Neighbors, deepClone(n, seen))
  }

  return &node_clone
}

func main() {
  node1 := Node { Val: 1 }
  node2 := Node { Val: 2 } 
  node3 := Node { Val: 3 } 

  node1.Neighbors = append(node1.Neighbors, &node2)
  node1.Neighbors = append(node1.Neighbors, &node3)
  node2.Neighbors = append(node2.Neighbors, &node1)

  newGraph := cloneGraph(&node1)

  fmt.Println(newGraph)
}
