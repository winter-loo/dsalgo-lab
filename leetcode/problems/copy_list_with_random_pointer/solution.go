// go run solution.go
package main

import "fmt"

type Node struct {
    Val int
    Next *Node
    Random *Node
}

func (this *Node) String() string {
  self := this
  var out = ""
  for this != nil {
    out += fmt.Sprintf("%p:%v", this, this.Val)
    if this.Next != nil {
      out += fmt.Sprintf(" -> ")
    }
    this = this.Next
  }
  this = self
  out += fmt.Sprintf("\nrandom:\n")
  for this != nil {
    if this.Random != nil {
      out += fmt.Sprintf("%p -> %p\n", this, this.Random)
    }
    this = this.Next
  }

  return out
}

func copyRandomList(head *Node) *Node {
  var dummy = Node{0, nil, nil}
  dummy.Next = head
  prev := &dummy

  var randoms = make(map[*Node][]int)
  var random_indicies = make(map[int]int)
  var new_nodes = make([]*Node, 0)

  i := 0
  for p := head; p != nil; p = p.Next {
    if p.Random != nil {
      randoms[p.Random] = append(randoms[p.Random], i)
    }
    i++
  }

  // fmt.Println("randoms")
  // for key, val := range randoms {
  //   fmt.Printf("%p -> %v\n", key, val)
  // }

  i = 0
  for p := head; p != nil; p = p.Next {
    if indicies, ok := randoms[p]; ok {
      for j := 0; j < len(indicies); j++ {
        random_indicies[indicies[j]] = i
      }
    }
    i++
  }

  // fmt.Println("random indicies")
  // fmt.Println(random_indicies)

  for i := 0; head != nil; i++ {
    var current = Node{0, nil,nil}
    new_nodes = append(new_nodes, &current)
    current.Val = head.Val
    prev.Next = &current
    prev = &current
    head = head.Next
  }

  for from, to := range random_indicies {
    new_nodes[from].Random = new_nodes[to]
  }
  return dummy.Next
}

func buildList(data [][]interface{}) *Node {
    n := len(data)
    nodes := make([]*Node, n)

    // Create all nodes
    for i, item := range data {
        val := item[0].(int)
        nodes[i] = &Node{Val: val}
    }

    // Set next and random pointers
    for i := 0; i < n; i++ {
        if i < n-1 {
            nodes[i].Next = nodes[i+1]
        }
        if data[i][1] != nil {
            randIndex := data[i][1].(int)
            nodes[i].Random = nodes[randIndex]
        }
    }

    return nodes[0]
}

func main() {
   data := [][]interface{}{
        {1, nil},
        {2, 0},
        {3, 4},
        {4, 2},
        {5, 0},
   }
   head := buildList(data)

   fmt.Println(head)
   new_head := copyRandomList(head)
   fmt.Println(new_head)
}
