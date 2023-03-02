/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// You are given an array nums of n positive integers.
//
// You can perform two types of operations on any element of the array any number of times:
//
// If the element is even, divide it by 2.
// For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,2].
// If the element is odd, multiply it by 2.
// For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [2,2,3,4].
// The deviation of the array is the maximum difference between any two elements in the array.
//
// Return the minimum deviation the array can have after performing some number of operations.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: 1
// Explanation: You can transform the array to [1,2,3,2], then to [2,2,3,2], then the deviation will be 3 - 2 = 1.
// Example 2:
//
// Input: nums = [4,1,5,20,3]
// Output: 3
// Explanation: You can transform the array after two operations to [4,2,5,5,3], then the deviation will be 5 - 2 = 3.
// Example 3:
//
// Input: nums = [2,10,8]
// Output: 3
//
//
// Constraints:
//
// n == nums.length
// 2 <= n <= 5 * 104
// 1 <= nums[i] <= 109

use std::collections::BinaryHeap;
pub  mod  a_1675{
    use std::collections::BinaryHeap;

    pub  fn  minimum_deviation(nums:Vec<i32>) -> i32{
        let  mut ans:i32 = i32::MAX;
        let  mut pq:BinaryHeap<i32> = BinaryHeap::new();
        let  mut mn = i32::MAX;
        for idx in 0 .. nums.len()  {
            if  nums[idx]%2 ==0{
                pq.push(nums[idx]);
                mn = mn.min(nums[idx]);
            } else{
                pq.push(nums[idx]*2);
                mn =mn.min(nums[idx]*2);
            }
        }
        while !pq.is_empty() {
            let  top = pq.pop().unwrap();
            ans = ans.min(top - mn);
            if top%2 !=0 {
                break;
            }
            mn = mn.min(top/2);
            pq.push(top/2);
        }

        ans
    }
}



#[cfg(test)]
mod  test{
    use crate::general_problems::a_01675_minimize_deviation_in_array::a_1675::minimum_deviation;

    #[test]
    fn t_001(){
        let  inp:Vec<i32> = vec![1,2,3,4];
        let  ans:i32 = 1;
        assert_eq!(ans, minimum_deviation(inp));
    }


    #[test]
    fn t_002(){
        let  inp:Vec<i32> = vec![4,1,5,20,3];
        let  ans:i32 = 3;

        assert_eq!(ans, minimum_deviation(inp));
    }

    #[test]
    fn t_003(){
        let  inp:Vec<i32> = vec![2,10,8];
        let  ans:i32 = 3;

        assert_eq!(ans, minimum_deviation(inp));
    }
}