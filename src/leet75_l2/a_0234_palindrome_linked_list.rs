/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/palindrome-linked-list
//
// Given the head of a singly linked list, return true if it is a
// palindrome
// or false otherwise.
//
//
//
// Example 1:
//
//
// Input: head = [1,2,2,1]
// Output: true
// Example 2:
//
//
// Input: head = [1,2]
// Output: false
//
//
// Constraints:
//
// The number of nodes in the list is in the range [1, 105].
// 0 <= Node.val <= 9
//
//
// Follow up: Could you do it in O(n) time and O(1) space?

pub  mod a234{
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let  mut this_head = head.clone();
        let  mut arr:Vec<i32> = vec![];
        while let Some (mut node) = this_head{
            arr.push(node.val);
            this_head = node.next;
            if  this_head.is_none(){break }

        }
        if arr.len() ==0{
            return true;
        }
        if arr.len() ==1{
            return  true;
        }

        let  mut pos1:usize =0;
        let  mut pos2 = arr.len()-1;
        let  mx_ctr = (arr.len()-1)/2;


        while pos1 <= mx_ctr {
            if  arr.get(pos1).unwrap() != arr.get(pos2).unwrap()  {
                return false;
            }
            pos1 +=1;
            pos2 -=1;
        }

        true
    }


    pub fn is_palindrome_faster(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut v = Vec::new();

        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }

        for left in 0..v.len() / 2 {
            let right = v.len() - left - 1;
            if right <= left {
                break;
            }

            if v[left] != v[right] {
                return false;
            }
        }

        true

    }
}


#[cfg(test)]
mod test{
    use crate::leet75_l2::a_0234_palindrome_linked_list::a234::{is_palindrome, ListNode};

    #[test]
    fn t_000(){
        let n = ListNode::new(20);
        assert_eq!(is_palindrome(Some(Box::new(n))), true);
    }

    #[test]
    fn t_001(){
        let n = ListNode::new(2);
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        assert_eq!(is_palindrome(Some(Box::new(n))), false);
    }

    #[test]
    fn t_002(){
        let n = ListNode::new(20);
        assert_eq!(is_palindrome(Some(Box::new(n))), true);
        let n = ListNode::new(2);
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        assert_eq!(is_palindrome(Some(Box::new(n))), false);

    }
    #[test]
    fn  t_003(){
        let n = ListNode::new(1);
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        assert_eq!(is_palindrome(Some(Box::new(n))), true);

    }
    #[test]
    fn t_004(){
        assert_eq!(is_palindrome(None), true);
    }
}