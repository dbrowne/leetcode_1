/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/single-number/
// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//
//
//
// Example 1:
//
// Input: nums = [2,2,1]
// Output: 1
// Example 2:
//
// Input: nums = [4,1,2,1,2]
// Output: 4
// Example 3:
//
// Input: nums = [1]
// Output: 1
//
//
// Constraints:
//
// 1 <= nums.length <= 3 * 104
// -3 * 104 <= nums[i] <= 3 * 104
// Each element in the array appears twice except for one element which appears only once.

pub  mod  a136{
    pub  fn  single_number(nums:Vec<i32>) ->i32{
        let  mut ans =0;
        for val  in nums  {
            ans ^=val;
        }
        ans
    }
}


#[cfg(test)]
mod  test{
    use crate::leet_75_l3::a_136_single_number::a136::single_number;

    #[test]
    fn t_000(){
        let  inp =vec![2,2,1];
        let  ans = 1;

        assert_eq!(ans, single_number(inp));
    }

    #[test]
    fn t_001(){
        let  inp =vec![4,1,2,1,2];
        let  ans = 4;
        assert_eq!(ans, single_number(inp));
    }

    #[test]
    fn t_002(){
        let  inp = vec![1];
        let  ans = 1;
        assert_eq!(ans, single_number(inp));
    }
}