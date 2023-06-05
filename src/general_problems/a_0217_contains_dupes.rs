/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: true
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: false
// Example 3:
//
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// -109 <= nums[i] <= 109


pub mod a217 {
    use std::collections::HashSet;
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let  mut hs:HashSet<i32> = HashSet::with_capacity(nums.len());
        for idx in nums.iter(){
            if hs.contains(idx){
                return true;
            }else{
                hs.insert(*idx);
            }
        }
        false
    }
}

#[cfg(test)]
mod  test{
    #[test]
    fn t_00(){
        let nums: Vec<i32> = vec![1,2,3,1];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,true);
    }

    #[test]
    fn t_01(){
        let nums: Vec<i32> = vec![1,2,3,4];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,false);
    }

    #[test]
    fn t_02(){
        let nums: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,true);
    }

    #[test]
    fn t_03(){
        let nums :Vec<i32> =vec![1000000000,1000000000,11];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,true);
    }

    #[test]
    fn t_04(){
       let nums:Vec<i32> = vec![1,2,3,4];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,false);
    }

    #[test]
    fn t_05(){
        let nums:Vec<i32>  =vec![1,5,-2,-4,0];
        let ans = super::a217::contains_duplicate(nums);
        assert_eq!(ans,false);
    }
}