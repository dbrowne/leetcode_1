/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given the head of a singly linked list where elements are sorted in ascending order, convert it to a
// height-balanced
// binary search tree.
//
//
//
// Example 1:
//
//
// Input: head = [-10,-3,0,5,9]
// Output: [0,-3,9,-10,null,5]
// Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
// Example 2:
//
// Input: head = []
// Output: []



pub mod a24 {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Definition for singly-linked list.
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

    // Definition for a binary tree node.
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }

    struct Solution {}

    impl Solution {
        pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
            let mut tree_size: i32 = 0;
            let mut node: &Option<Box<ListNode>> = &head;
            while let Some(ListNode {
                               next,
                               ..
                           }) = node.as_deref() {
                tree_size += 1;
                node = next;
            }
            fn gen_bst(head: &Option<Box<ListNode>>, size: i32) -> (Option<Rc<RefCell<TreeNode>>>,
                                                                    &Option<Box<ListNode>>) {
                match size {
                    0 => (None, head),
                    _ => {
                        let half_size = size / 2;

                        let (left,
                            root) = gen_bst(head, half_size);

                        let &ListNode {
                            ref next,
                            val
                        } = root.as_deref().unwrap();

                        let (right,
                            next) = gen_bst(next, size - half_size - 1);

                        (Some(Rc::new(RefCell::new(TreeNode { val, left, right }))), next)
                    }
                }
            }

            gen_bst(&head, tree_size).0



        }
    }
}