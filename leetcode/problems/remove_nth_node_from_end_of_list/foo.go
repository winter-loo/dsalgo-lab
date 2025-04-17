package main

import "fmt"
import "strings"

type ListNode struct {
  Val int
  Next *ListNode
}


// String implements the fmt.Stringer interface for pretty printing
func (node *ListNode) String() string {
	var sb strings.Builder
	for curr := node; curr != nil; curr = curr.Next {
		sb.WriteString(fmt.Sprintf("%d", curr.Val))
		if curr.Next != nil {
			sb.WriteString(" -> ")
		}
	}
	return sb.String()
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
  p1 := head
  if p1 == nil {
    return nil
  }

  for i := 0; i < n; i++ {
    if p1.Next != nil {
      p1 = p1.Next
    } else {
      break
    }
  }

  fmt.Println("p1: ", p1)

  p2 := head
  for p1.Next != nil {
    p1 = p1.Next
    p2 = p2.Next
  }
  fmt.Println("p2: ", p2)

  fmt.Printf("p2: %p, head: %p\n", p2, head)
  if n == 1 && p2 == head {
    return p2.Next
  }
  p2.Next = p2.Next.Next

  return head
}

func main() {
  head := ListNode{Val: 1}
  node1 := ListNode{Val: 2}

  head.Next = &node1

  fmt.Println("original:")
  fmt.Println(&head)

  var new_head = removeNthFromEnd(&head, 1)

  fmt.Println("removed")
  fmt.Println(new_head)
}
