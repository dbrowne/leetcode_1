/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/invert-binary-tree/
//
//
// Example 1:
//
//
// Input: root = [4,2,7,1,3,6,9]
// Output: [4,7,2,9,6,3,1]
// Example 2:
//
//
// Input: root = [2,1,3]
// Output: [2,3,1]
// Example 3:
//
// Input: root = []
// Output: []
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100


pub mod a226 {

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

    use std::rc::Rc;
    use std::cell::RefCell;

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn invert_tree(root: Node) -> Node {
            if let Some(node) = root.clone() {
                let mut node = node.borrow_mut();
                let (l, r) = (
                    Self::invert_tree(node.left.clone()),
                    Self::invert_tree(node.right.clone())
                );
                node.left = r;
                node.right = l;
            }
            root
        }

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn t_000(){
        assert_eq!(0,-1 );
    }
}