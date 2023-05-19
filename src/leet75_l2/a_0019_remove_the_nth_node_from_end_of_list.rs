/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/remove-nth-node-from-end-of-list
//
// Given the head of a linked list, remove the nth node from the end of the list and return its head.
//
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4,5], n = 2
// Output: [1,2,3,5]
// Example 2:
//
// Input: head = [1], n = 1
// Output: []
// Example 3:
//
// Input: head = [1,2], n = 1
// Output: [1]
//
//
// Constraints:
//
// The number of nodes in the list is sz.
// 1 <= sz <= 30
// 0 <= Node.val <= 100
// 1 <= n <= sz
//
//
// Follow up: Could you do this in one pass?

pub  mod a19{

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut buf = Vec::with_capacity(n as usize);
        let mut cnt = 0;
        let mut new_head = None;
        let mut new_node = &mut new_head;
        while let Some(mut node) = head {  // node takes ownership of the box
            if cnt < n { cnt += 1 }
            else {
                *new_node = Some(Box::new(ListNode::new(buf.remove(0))));
                new_node = &mut new_node.as_mut().unwrap().next;
            }
            head = node.next.take();    // head takes the ownership of the rest list
            buf.push(node.val);
        }   // the owned node (1 struct ListNode) dropped here
        buf.remove(0);
        for i in buf {
            *new_node = Some(Box::new(ListNode::new(i)));
            new_node = &mut new_node.as_mut().unwrap().next;
        }
        new_head
    }

    pub fn remove_nth_from_end_faster(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end_internal(head, n).0
    }
    fn remove_nth_from_end_internal(node: Option<Box<ListNode>>, n: i32) ->(Option<Box<ListNode>>, i32){
        match node {
            None => (None, 1),
            Some(mut curr) => {
                let (nxt, curr_id) = remove_nth_from_end_internal(curr.next.take(), n);
                if curr_id == n {
                    (nxt, curr_id + 1)
                } else {
                    curr.next = nxt;
                    (Some(curr), curr_id + 1)
                }
            }
        }
    }


}

#[cfg(test)]
mod test {
    use crate::leet75_l2::a_0019_remove_the_nth_node_from_end_of_list::a19::{ListNode, remove_nth_from_end_faster};

    #[test]
    fn t_000(){


        let n5 = ListNode { val: 5, next: None };
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head = Some(Box::new(n1));

        let n5 = ListNode { val: 5, next: None };
        let n3 = ListNode { val: 3, next: Some(Box::new(n5)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head_ = Some(Box::new(n1));

        assert_eq!(remove_nth_from_end_faster(head, 2), head_)
    }
}