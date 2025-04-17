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

  i := 0
  for ; i < n; i++ {
    if p1.Next != nil {
      p1 = p1.Next
    } else {
      break
    }
  }

  if i < n {
    if i + 1 == n {
      return head.Next
    }
    return head
  }

  // fmt.Println("p1: ", p1, i)

  p2 := head
  for p1.Next != nil {
    p1 = p1.Next
    p2 = p2.Next
  }
  // fmt.Println("p2: ", p2)

  p2.Next = p2.Next.Next

  return head
}

func createLinkedList(values []int) *ListNode {
	if len(values) == 0 {
		return nil
	}
	
	head := &ListNode{Val: values[0]}
	current := head
	
	for i := 1; i < len(values); i++ {
		current.Next = &ListNode{Val: values[i]}
		current = current.Next
	}
	
	return head
}


func main() {
  head := createLinkedList([]int{1, 2, 3, 4, 5})

  fmt.Println("original:")
  fmt.Println(head)

  var new_head = removeNthFromEnd(head, 2)

  fmt.Println("removed")
  fmt.Println(new_head)
}
