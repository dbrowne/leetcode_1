pub mod day_eight {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::cmp;
    use std::rc::Rc;

    type OptNode = Option<Rc<RefCell<TreeNode>>>;

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

    // pub fn lowest_common_ancestor(root: OptNode, p: OptNode, q: OptNode) -> OptNode {
    //     let p_val = p.as_ref().unwrap().borrow().val;
    //     let q_val = q.as_ref().unwrap().borrow().val;
    //     let (min, max) = match p_val > q_val {
    //         true => (q_val, p_val),
    //         false => (p_val, q_val),
    //     };
    //
    //     let mut stack = vec![root];
    //     while let Some(node) = stack.pop() {
    //         if let Some(node) = node {
    //             let node_val = node.borrow().val;
    //             if node_val >= min && node_val <= max {
    //                 return Some(node);
    //             }
    //             match node_val > max {
    //                 true => stack.push(node.borrow().left.clone()),
    //                 false => stack.push(node.borrow().right.clone()),
    //             }
    //         }
    //     }
    //     unreachable!()
    // }
    // pub fn lowest_common_ancestor_fast(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     match (root, p, q) {
    //         (Some(a), Some(r), Some(l)) => {
    //             if a.borrow().val > cmp::max(r.borrow().val, l.borrow().val) {
    //                 return lowest_common_ancestor_fast(
    //                     a.borrow().left.clone(),
    //                     Some(r.clone()),
    //                     Some(l.clone()),
    //                 );
    //             } else if a.borrow().val < cmp::min(r.borrow().val, l.borrow().val) {
    //                 return lowest_common_ancestor_fast(
    //                     a.borrow().right.clone(),
    //                     Some(r.clone()),
    //                     Some(l.clone()),
    //                 );
    //             } else {
    //                 Some(a.clone())
    //             }
    //         }
    //         (_, _, _) => None,
    //     }
    // }
}

// #[cfg(test)]
// mod test{
//     use crate::leetweektwo::day8::day_eight::is_valid_bst;
//     #[macro_use]    use crate::util::tree::{tree,to_tree};
//
//     // #[test]
//     // fn test_1(){
//     //      assert_eq!(is_valid_bst(tree![5,1,4,null, null,3,6]), false);
//     //  }
//
//
// }
