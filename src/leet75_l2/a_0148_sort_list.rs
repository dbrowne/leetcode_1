/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/sort-list/
//
// Given the head of a linked list, return the list after sorting it in ascending order.
//
//
//
// Example 1:
//
//
// Input: head = [4,2,1,3]
// Output: [1,2,3,4]
// Example 2:
//
//
// Input: head = [-1,5,3,4,0]
// Output: [-1,0,3,4,5]
// Example 3:
//
// Input: head = []
// Output: []
//
//
// Constraints:
//
// The number of nodes in the list is in the range [0, 5 * 104].
// -105 <= Node.val <= 105
//
//
// Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?

pub  mod a148{
    #[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            len += 1;
            ptr = node.next.as_ref();
        }
        merge_sort(head, len)
    }

    fn merge_sort(mut head: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
        if len < 2 {
            return head;
        }
        let mut next = head.as_mut();
        let mut i = 1;
        while i < len / 2 {
            next = next.unwrap().next.as_mut();
            i += 1;
        }
        let mut l2 = next.unwrap().next.take();
        let mut l1 = merge_sort(head, len / 2);
        let mut l2 = merge_sort(l2, len - len / 2);
        merge(l1, l2)
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        loop {
            match (l1, l2) {
                (Some(mut node1), Some(mut node2)) => {
                    let node = if node1.val > node2.val {
                        // give back ownership
                        l2 = node2.next.take();
                        l1 = Some(node1);
                        node2
                    } else {
                        l1 = node1.next.take();
                        l2 = Some(node2);
                        node1
                    };
                    next.as_mut().unwrap().next = Some(node);
                    next = next.unwrap().next.as_mut();
                }
                (Some(mut node1), None) => {
                    next.unwrap().next = Some(node1);
                    break;
                }
                (None, Some(mut node2)) => {
                    next.unwrap().next = Some(node2);
                    break;
                }
                (None, None) => break,
            }
        }
        dummy.unwrap().next
    }


    ////////////////--------------------------------------------------------------
    pub fn get_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();

        while let Some(node) = head {
            out.push(node.val);
            head = node.next;
        }

        out
    }

    pub fn get_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            return None;
        }

        let mut root = Box::new(ListNode::new(nums[0]));
        let mut node_ref = &mut root;

        for n in nums.iter().skip(1) {
            node_ref.next = Some(Box::new(ListNode::new(*n)));

            node_ref = node_ref.next.as_mut().unwrap();
        }

        Some(root)
    }

    pub fn sort_list_faster(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut out = get_vec(head);

        out.sort_unstable();

        get_list(out)
    }

    ////////////////--------------------------------------------------------------



}

#[cfg(test)]
mod test{
    use crate::leet75_l2::a_0148_sort_list::a148::ListNode;

    #[test]
    fn t_001(){
        let n5 = ListNode { val: 5, next: None };
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head = Some(Box::new(n1));
        assert_eq!(0,-1 );
    }
}