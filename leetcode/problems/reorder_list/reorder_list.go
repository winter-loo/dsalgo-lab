// go build reorder_list.go
package main

import "fmt"

type ListNode struct {
  Val int
  Next *ListNode
}

func (this *ListNode) String() string {
  var result = ""
  for this != nil {
    result += fmt.Sprintf("%v", this.Val)
    if this.Next != nil {
      result += fmt.Sprintf("%v", " -> ")
    }
    this = this.Next
  }
  return result
}

func reverse(this *ListNode) *ListNode {
  next := this
  var p *ListNode = nil
  for {
    // fmt.Println(next.Val)
    tmp := next.Next
    next.Next = p
    if tmp == nil {
      break
    }
    p = next
    next = tmp
  }
  return next
}

func reorderList(head *ListNode) {
  if head == nil || head.Next == nil {
    return
  }
  slow, fast := head, head.Next
  for fast != nil && fast.Next != nil {
    fast = fast.Next.Next
    slow = slow.Next
  }
  tmp := slow.Next
  slow.Next = nil
  slow = tmp
  
  second_half := reverse(slow)

  l1 := head
  l2 := second_half
  // fmt.Println("l1", l1)
  // fmt.Println("l2", l2)

  for l2 != nil {
    tmp := l1.Next
    l1.Next = l2
    l1 = tmp
    tmp = l2.Next
    l2.Next = l1
    l2 = tmp
  }

  // fmt.Println("head", head)
}

func main() {
  head := ListNode{1, nil}
  node1 := ListNode{2, nil}
  node2 := ListNode{3, nil}
  node3 := ListNode{4, nil}
  node4 := ListNode{5, nil}
  head.Next = &node1
  node1.Next = &node2
  node2.Next = &node3
  node3.Next = &node4

  // fmt.Println(head.Reverse())

  reorderList(&head)
}
