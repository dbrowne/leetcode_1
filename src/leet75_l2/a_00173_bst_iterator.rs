/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/binary-search-tree-iterator/
//
// Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):
//
// BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
// boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
// int next() Moves the pointer to the right, then returns the number at the pointer.
// Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.
//
// You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.
//
//
//
// Example 1:
//
//
// Input
// ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
// [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
// Output
// [null, 3, 7, true, 9, true, 15, true, 20, false]
//
// Explanation
// BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
// bSTIterator.next();    // return 3
// bSTIterator.next();    // return 7
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 9
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 15
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 20
// bSTIterator.hasNext(); // return False
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 105].
// 0 <= Node.val <= 106
// At most 105 calls will be made to hasNext, and next.
//
//
// Follow up:
//
// Could you implement next() and hasNext() to run in average O(1) time and use O(h) memory, where h is the height of the tree?

pub  mod  a173 {

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
                right: None
            }
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    struct BSTIterator {
        stack: Vec<Rc<RefCell<TreeNode>>>,
        cur: Option<Rc<RefCell<TreeNode>>>,
    }

    impl BSTIterator {
        fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
            BSTIterator {
                stack: vec![],
                cur: root,
            }
        }

        fn next(&mut self) -> i32 {
            while self.cur.is_some() {
                let cur = self.cur.take().unwrap();
                let left = cur.borrow_mut().left.take();
                self.stack.push(cur);
                self.cur = left;
            }

            let cur = self.stack.pop().unwrap();
            let val = cur.borrow().val;
            self.cur = cur.borrow_mut().right.take();
            val
        }

        fn has_next(&self) -> bool {
            self.cur.is_some() || !self.stack.is_empty()
        }
    }


    // Faster Solution

    struct BSTIterator2 {
        stack: Vec<StackValue>,
    }

    enum StackValue {
        Node(Rc<RefCell<TreeNode>>),
        Value(i32)
    }

    /**
      * `&self` means the method takes an immutable reference
      * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator2 {

        fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
            Self {
                stack: if let Some(root) = root { vec![StackValue::Node(root)] } else { vec![] },
            }
        }

        fn next(&mut self) -> i32 {
            while let Some(item) = self.stack.pop() {
                match item {
                    StackValue::Value(val) => return val,
                    StackValue::Node(node) => {
                        let item = node.borrow();

                        if let Some(right) = item.right.as_ref() {
                            self.stack.push(StackValue::Node(right.clone()));
                        }
                        self.stack.push(StackValue::Value(item.val));
                        if let Some(left) = item.left.as_ref() {
                            self.stack.push(StackValue::Node(left.clone()));
                        }
                    }
                }
            }

            return -1;
        }

        fn has_next(&self) -> bool {
            !self.stack.is_empty()
        }
    }

}