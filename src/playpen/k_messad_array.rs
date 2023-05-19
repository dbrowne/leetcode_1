/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of integers arr where each element is at most k places away from its sorted position,
// code an efficient function sortKMessedArray that sorts arr. For instance, for an input array of size 10 and k = 2,
// an element belonging to index 6 in the sorted array will be located at either index 4, 5, 6, 7 or 8 in the input array.
//
// Analyze the time and space complexities of your solution.
//
// Example:
//
// input:  arr = [1, 4, 5, 2, 3, 7, 8, 6, 10, 9], k = 2
//
// output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

pub  mod  aaa{
    use std::collections::BinaryHeap;
    pub  fn sort_k_messed_array(arr:Vec<i32>, k:i32) ->Vec<i32>{

        let  al:usize = arr.len();
        let  sz:usize = k as usize +1;

        let  mut hp:BinaryHeap<i32> = BinaryHeap::with_capacity(sz);
        let  mut ans:Vec<i32> = Vec::with_capacity(al);

        for idx in 0 .. (sz )  {
            hp.push(arr[idx]*-1);
        }

        for idx in sz .. (al)  {
            ans.push(hp.pop().unwrap() * -1);
            hp.push(arr[idx] * -1);
        }
        let  mut ctr:usize = 0;
        while !hp.is_empty() {
            ans.push(hp.pop().unwrap()* -1);
        }
        ans
    }
}

#[cfg(test)]
mod  test{
    use crate::playpen::k_messad_array::aaa::sort_k_messed_array;

    #[test]
    fn t00(){
        let  inp:Vec<i32> = vec![2, 1, 4, 5, 3, 7, 8, 6, 10, 9];
        let  k = 2;
        let  ans:Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let  ret:Vec<i32> = sort_k_messed_array(inp, k);

        assert_eq!(ans,ret );
    }
}
