/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
//
// Example 1:
//
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]
// Example 2:
//
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]
// Example 3:
//
// Input: nums = [], target = 0
// Output: [-1,-1]
//
//
// Constraints:
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109
// nums is a non-decreasing array.
// -109 <= target <= 109



pub mod a34 {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![-1, -1];
        if nums.is_empty() {
            return ans;
        }
        ans[0] = search(&nums, &target, true);
        ans[1] = search(&nums, &target, false);
        ans
    }

    pub fn search(nums: &Vec<i32>, target: &i32, left_param: bool) -> i32 {
        let mut ans: i32 = -1;
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if left_param {
                if nums[mid as usize] >= *target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid as usize] <= *target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            if nums[mid as usize] == *target {
                ans = mid as i32;
            }
        }
        ans
    }
}

    #[cfg(test)]
    mod test {
        #[test]
        fn t_01() {
            assert_eq!(vec![3, 4], super::a34::search_range(vec![5, 7, 7, 8, 8, 10], 8));
        }

        #[test]
        fn t_02() {
            assert_eq!(vec![-1, -1], super::a34::search_range(vec![5, 7, 7, 8, 8, 10], 6));
        }

        #[test]
        fn t_03() {
            assert_eq!(vec![-1, -1], super::a34::search_range(vec![], 0));
        }
        #[test]
        fn t_04(){
            assert_eq!(vec![-1, -1], super::a34::search_range(vec![1], 0));
        }
        #[test]
        fn t_05(){
            assert_eq!(vec![-1, -1], super::a34::search_range(vec![2,2], 1));
        }
    }