package main

import "fmt"

type ListNode struct {
    Val int
    Next *ListNode
}

func (this *ListNode) String() string {
  var out = ""
  for this != nil {
    out += fmt.Sprintf("%v", this.Val)
    if this.Next != nil {
      out += fmt.Sprintf(" -> ")
    }
    this = this.Next
  }

  return out
}

func reverseKGroup(head *ListNode, k int) *ListNode {
  if head == nil || head.Next == nil {
    return head
  }
  var new_head *ListNode
  var prev_tail *ListNode
  p := head
  for p != nil && p.Next != nil {
    i := 0
    for ; i < k - 1 && p.Next != nil; i++ {
      p = p.Next
    }
    if i < k - 1 {
      break
    }
    // fmt.Println("p.Val", p.Val, "head.Val", head.Val)
    next := p.Next
    p.Next = nil
    rev := reverse(head)
    if new_head == nil {
      new_head = rev
    }
    if prev_tail != nil {
      prev_tail.Next = rev
    }
    prev_tail = head
    prev_tail.Next = next
    head = next
    p = head
  }
  return new_head
}

func reverse(head *ListNode) *ListNode {
  var p *ListNode
  next := head

  for next != nil {
    tmp := next.Next
    next.Next = p
    p = next
    next = tmp
  }
  return p
}

func buildList(values []int) *ListNode {
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
  head := buildList([]int{1, 2, 3, 4, 5})
  fmt.Println(head)
  fmt.Println(reverseKGroup(head, 1))
}
