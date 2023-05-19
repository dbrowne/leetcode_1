/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
//
//
//
// Example 1:
//
//
// Input: head = [1,2,3,4]
// Output: [2,1,4,3]
// Example 2:
//
// Input: head = []
// Output: []
// Example 3:
//
// Input: head = [1]
// Output: [1]
//
//
// Constraints:
//
// The number of nodes in the list is in the range [0, 100].
// 0 <= Node.val <= 100

pub mod a24 {

    // #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val,
            }
        }
    }


    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list:ListNode = ListNode::new(0);
        let mut tail:&mut Option<Box<ListNode>> = &mut list.next;

        let mut temp = None;

        while let Some(mut node) = head.take() {

            head = node.next.take();

            match temp.take() {
                None => temp = Some(node),
                Some(temp) => {
                    node.next = Some(temp);
                    *tail = Some(node);
                    tail = &mut tail.as_mut().unwrap().next.as_mut().unwrap().next;
                }
            }
        }

        *tail = temp;
        list.next.take()
    }
}