/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
//
//
//
// Example 1:
//
//
// Input: root = [3,9,20,null,null,15,7]
// Output: [[3],[20,9],[15,7]]
// Example 2:
//
// Input: root = [1]
// Output: [[1]]
// Example 3:
//
// Input: root = []
// Output: []
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 2000].
// -100 <= Node.val <= 100

pub mod a_103 {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::VecDeque;

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

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<VecDeque<i32>>, dpt: usize) {
            if let Some(r) = root {
                if levels.len() == dpt {
                    levels.push(VecDeque::new())
                }
                let r = r.borrow();
                if dpt % 2 == 0 {
                    levels[dpt].push_back(r.val);
                } else {
                    levels[dpt].push_front(r.val);
                }
                dfs(r.left.clone(), levels, dpt + 1);
                dfs(r.right.clone(), levels, dpt + 1);
            }
        }

        dfs(root, &mut ans, 0);
        ans.into_iter()
           .map(|d| d.into_iter().collect::<Vec<_>>())
           .collect()
    }

}


