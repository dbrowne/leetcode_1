/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

//
// https://leetcode.com/problems/maximum-product-subarray/
// Given an integer array nums, find a
// subarray
// that has the largest product, and return the product.
//
// The test cases are generated so that the answer will fit in a 32-bit integer.
//
//
//
// Example 1:
//
// Input: nums = [2,3,-2,4]
// Output: 6
// Explanation: [2,3] has the largest product 6.
// Example 2:
//
// Input: nums = [-2,0,-1]
// Output: 0
// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
//
//
// Constraints:
//
// 1 <= nums.length <= 2 * 104
// -10 <= nums[i] <= 10
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

pub  mod  a152{
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() ==0{
            return 0;
        }
        let  nl = nums.len();
        let  mut res = nums[0];
        let mut cur_max = nums[0];
        let  mut cur_min = nums[0];

        for idx  in 1..nl  {
            let   curr = nums[idx];
            let  tmp_mx = std::cmp::max(cur_max*curr, cur_min*curr);
            cur_min = std::cmp::max(cur_max*curr, cur_min*curr);

            cur_max = tmp_mx;
            res = std::cmp::max(cur_max,res);
        }

    res
    }


    pub fn max_product_faster(nums: Vec<i32>) -> i32 {
        let mut max = nums.clone();
        let mut min = nums.clone();

        for i in 1..nums.len() {
            max[i] = i32::max(i32::max(nums[i], min[i - 1] * nums[i]), max[i - 1] * min[i]);
            min[i] = i32::min(i32::min(nums[i], min[i - 1] * nums[i]), max[i - 1] * min[i]);
        }

        max.iter().max().or(Some(&0)).unwrap().clone()
    }
}

#[cfg(test)]
mod  test {
    use crate::leet75_L2::a_00152_maximum_product_subarray::a152::max_product;

    #[test]
    fn t_001(){
        let inp = vec![2,3,-2,4];
        assert_eq!(6,max_product(inp) );
    }
}