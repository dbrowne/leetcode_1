/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub mod a100 {
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

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::LinkedList;

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(ref m), Some(ref n)) => {
                let m = m.borrow();
                let n = n.borrow();
                m.val == n.val && is_same_tree(n.left.clone(), m.left.clone()) &&
                    is_same_tree(n.right.clone(), m.right.clone())
            }
        }
    }


    pub fn is_same_tree_faster(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue: LinkedList<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> = LinkedList::from([
            (p, q)
        ]);

        //Self::check(&p, &q)


        while !queue.is_empty() {
            let (a, b) = queue.pop_back().unwrap();

            if let Some(a) = a {
                if let Some(b) = b {
                    let mut a = a.borrow_mut();
                    let mut b = b.borrow_mut();
                    if a.val == b.val {
                        queue.push_front( (a.left.take(), b.left.take()) );
                        queue.push_front( (a.right.take(), b.right.take()) );

                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                if let Some(_) = b {
                    return false;
                }
            }
        }

        true
    }



}


#[cfg(test)]
mod test {
    use crate::leet75_l2::a_00100_same_tree::a100::is_same_tree;

    #[test]
    fn t_000() {
        assert_eq!(true, is_same_tree(None, None));
    }
}