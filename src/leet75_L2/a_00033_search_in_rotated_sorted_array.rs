/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/search-in-rotated-sorted-array/
// There is an integer array nums sorted in ascending order (with distinct values).
//
// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
//
// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
//
// Example 1:
//
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
// Example 2:
//
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1
// Example 3:
//
// Input: nums = [1], target = 0
// Output: -1
//
//
// Constraints:
//
// 1 <= nums.length <= 5000
// -104 <= nums[i] <= 104
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -104 <= target <= 104


pub  mod  a33{
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let  mut start: usize = 0;
        let  mut end :usize=nums.len() -1;

        while start <= end {
            let  mut mid = start + (end-start)/2;
            if  nums[mid]==target{return mid as i32; }
            else if nums[mid] >= nums[start] {
                if  target >= nums[start] && target <nums[mid]{end= mid -1; }
                else { start = mid +1; }
            }else {
                if target <= nums[end] && target > nums[mid] { start = mid+1;}
                else { end = mid -1;  }
            }
        }


        -1
    }
}


#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00033_search_in_rotated_sorted_array::a33::search;

    #[test]
    fn t_000(){
        let  inp:Vec<i32> = vec![4,5,6,7,0,1,2];
        assert_eq!(-1,search(inp,3 ));
    }
    #[test]
    fn  t_001(){
        let  inp:Vec<i32> = vec![4,5,6,7,0,1,2];
        assert_eq!(4,search(inp,0));
    }
}