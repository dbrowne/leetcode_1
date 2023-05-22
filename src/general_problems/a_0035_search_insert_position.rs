/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
//
// Example 1:
//
// Input: nums = [1,3,5,6], target = 5
// Output: 2
// Example 2:
//
// Input: nums = [1,3,5,6], target = 2
// Output: 1
// Example 3:
//
// Input: nums = [1,3,5,6], target = 7
// Output: 4
//
//
// Constraints:
//
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums contains distinct values sorted in ascending order.
// -104 <= target <= 104



pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let ln = nums.len() as i32 -1;


    if target <= nums[0] {
        return 0;
    }

    if  target > nums[nums.len() -1]{
        return nums.len() as i32;
    }

    let mut  right = ln;
    let mut  left = 0;
    let  mut mid:i32 =0;

    while left <right {
        mid = left + (right -left)/2;
        if  nums[mid as usize] == target{
            return  mid;
        }
        if target < nums[mid as usize] {
            right =mid;
        }else{
            left = mid +1;
        }
    }

    if target < nums[mid as usize] {
        return mid ;
    }
    return mid +1;
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0035_search_insert_position::search_insert;

    #[test]
    fn t_0() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(2, search_insert(nums, target));
    }

    #[test]
    fn t_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(1, search_insert(nums, target));
    }

    #[test]
    fn t_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(4, search_insert(nums, target));
    }

    #[test]
    fn t_3(){
         let  nums = vec![1,3,5];
        let  target = 4;

        assert_eq!(2, search_insert(nums,target));
    }

    #[test]
    fn t_4(){
        let  nums = vec![2,7,8,9,10];
        let  target = 9;

        assert_eq!(3,search_insert(nums,target) );
    }
}