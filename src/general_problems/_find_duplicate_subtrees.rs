/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given the root of a binary tree, return all duplicate subtrees.
//
// For each kind of duplicate subtrees, you only need to return the root node of any one of them.
//
// Two trees are duplicate if they have the same structure with the same node values.
//
//
//
// Example 1:
//
//
// Input: root = [1,2,3,4,null,2,4,null,null,4]
// Output: [[2,4],[4]]
// Example 2:
//
//
// Input: root = [2,1,1]
// Output: [[1]]
// Example 3:
//
//
// Input: root = [2,2,2,3,null,3,null]
// Output: [[2,3],[3]]
//
//
// Constraints:
//
// The number of the nodes in the tree will be in the range [1, 5000]
// -200 <= Node.val <= 200

pub mod a652 {
    use std::rc::Rc;
    use std::collections::HashMap;
    use std::cell::RefCell;

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

    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![];
        let mut visited: HashMap<String, u8> = HashMap::new();

        fn traverse(
            root: Option<Rc<RefCell<TreeNode>>>, visited: &mut HashMap<String, u8>,
            ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>)
            -> String {
            if let Some(ref nodeRef) = root {
                let node = nodeRef.borrow();
                let l_node: String = traverse(node.left.clone(), visited, ans);
                let r_node: String = traverse(node.right.clone(), visited, ans);
                let tree: String = format!("{}<{}>{}", node.val, l_node, r_node);
                let see: &mut u8 = visited.entry(tree.clone()).or_default();
                *see += 1;
                if *see == 2 {
                    ans.push(root.clone());
                }
                tree.to_string()
            } else {
                "".to_string()
            }
        }
        traverse(root.clone(), &mut visited, &mut ans);
        ans
    }
}