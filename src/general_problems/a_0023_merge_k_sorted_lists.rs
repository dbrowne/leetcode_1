/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//
// Merge all the linked-lists into one sorted linked-list and return it.
//
//
//
// Example 1:
//
// Input: lists = [[1,4,5],[1,3,4],[2,6]]
// Output: [1,1,2,3,4,4,5,6]
// Explanation: The linked-lists are:
// [
// 1->4->5,
// 1->3->4,
// 2->6
// ]
// merging them into one sorted list:
// 1->1->2->3->4->4->5->6
// Example 2:
//
// Input: lists = []
// Output: []
// Example 3:
//
// Input: lists = [[]]
// Output: []
//
//
// Constraints:
//
// k == lists.length
// 0 <= k <= 104
// 0 <= lists[i].length <= 500
// -104 <= lists[i][j] <= 104
// lists[i] is sorted in ascending order.
// The sum of lists[i].length will not exceed 104.

pub mod a23 {
    // Definition for singly-linked list.
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


    use std::collections::BinaryHeap;
    use std::cmp::Ordering;

    impl PartialOrd<ListNode> for ListNode {
        fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
            other.val.partial_cmp(&self.val)
        }
    }

    impl Ord for ListNode {
        fn cmp(&self, other: &Self) -> Ordering {
            other.val.cmp(&self.val)
        }

    }


    pub  fn  merge_k_lists(lists: Vec<Option<Box<ListNode>>>)->Option<Box<ListNode>>{
        let  mut list_heap :BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());
        for sub_list in lists{
            match sub_list {
                Some(node) =>list_heap.push(node),
                None => {}
            }
        }
        let  mut temp_node:Box<ListNode> = Box::new(ListNode::new(0));
        let  mut this_node :&mut Box<ListNode>= &mut temp_node;

        while let Some(node) = list_heap.pop() {
            let  mut new_node = Box::new(ListNode::new(node.val));
            this_node.next = Some(new_node);
            this_node = this_node.next.as_mut().unwrap();
            if  node.next.is_some(){
                list_heap.push(node.next.unwrap());
            }
            
        }
        temp_node.next
    }



    pub  fn listnode_from_vec(vec: &Vec<i32>) -> Option<Box<ListNode>>
    {
        let mut dummy_node = ListNode::new(0);
        let mut curr = &mut dummy_node;

        for &v in vec
        {
            let node = ListNode::new(v);
            curr.next = Some(Box::new(node));
            curr = curr.next.as_mut().unwrap();
        }

        return dummy_node.next;
    }

    pub  fn listnode_to_vec(listnode: &Option<Box<ListNode>>) -> Vec<i32>
    {
        let mut vec: Vec<i32> = Vec::new();
        let mut curr = listnode;
        loop
        {
            match curr
            {
                Some(ref node) => vec.push(node.val),
                None => break
            }
            curr = &(curr.as_ref().unwrap().next);
        }
        return vec;
    }

    pub  fn print_listnode(listnode: &Option<Box<ListNode>>)
    {
        let mut curr = listnode;
        loop
        {
            match curr
            {
                Some(ref node) => print!("{} ", node.val),
                None => break
            }
            curr = &(curr.as_ref().unwrap().next);
        }
        println!("\n");
    }
}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_0023_merge_k_sorted_lists::a23::{ListNode, listnode_from_vec, merge_k_lists,print_listnode};

    #[test]
    fn t_000(){
        let vec_1: Vec<i32> = Vec::from([1,2,6]);
        let vec_2: Vec<i32> = Vec::from([2,4]);
        let vec_3: Vec<i32> = Vec::from([-10,5,20]);

        let l1: Option<Box<ListNode>> = listnode_from_vec(&vec_1);
        let l2: Option<Box<ListNode>> = listnode_from_vec(&vec_2);
        let l3: Option<Box<ListNode>> = listnode_from_vec(&vec_3);

        let mut lists: Vec<Option<Box<ListNode>>> = Vec::new();
        lists.push(l1);
        lists.push(l2);
        lists.push(l3);

        let sorted = merge_k_lists(lists);
        print_listnode(&sorted);
    }
}