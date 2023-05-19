/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */



pub  mod a328{

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


    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let (mut ret, mut part_two) = (None, None);
        let (mut tail1, mut tail2) = (&mut ret, &mut part_two);
        let mut flag = 0i32;

        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            flag = 1 - flag;
            if flag == 1 { tail1 = &mut tail1.insert(node).next; }
            else         { tail2 = &mut tail2.insert(node).next; }
        }

        if let Some(node) = part_two { tail1 = &mut tail1.insert(node).next; }

        ret
    }

    pub fn odd_even_list_faster(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut odd = head;
        let mut even = odd.as_mut().unwrap().next.take();

        let mut odd_tail = odd.as_mut();
        let mut even_tail = even.as_mut();
        while even_tail.is_some() && even_tail.as_ref().unwrap().next.is_some() {
            let next_even = even_tail.as_mut().unwrap().next.as_mut().unwrap().next.take();
            let next_odd = even_tail.as_mut().unwrap().next.take();

            even_tail.as_mut().unwrap().next = next_even;
            odd_tail.as_mut().unwrap().next = next_odd;

            even_tail = even_tail.unwrap().next.as_mut();
            odd_tail = odd_tail.unwrap().next.as_mut();
        }
        odd_tail.unwrap().next = even;
        odd
    }

}


#[cfg(test)]
mod test{
    #[test]
    fn t_000(){

    }
}