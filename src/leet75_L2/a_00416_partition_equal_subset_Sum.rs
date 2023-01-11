/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */
//
// https://leetcode.com/problems/partition-equal-subset-sum/
// Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
//
//
//
// Example 1:
//
// Input: nums = [1,5,11,5]
// Output: true
//
// Explanation: The array can be partitioned as [1, 5, 5] and [11].
// Example 2:
//
// Input: nums = [1,2,3,5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.
//
//
// Constraints:
//
// 1 <= nums.length <= 200
// 1 <= nums[i] <= 100



mod  a416{
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total = nums.iter().copied().sum::<i32>();

        // Fast path: either there are no elements in the vector or
        // their sum is not divisible by 2, thus it cannot be divided
        // evenly
        if total == 0 || total % 2 != 0 {
            return false;
        }

        let mut nums = nums.clone();
        nums.sort_unstable_by_key(|&x| -x);
        backtrack(&nums, total / 2, 0)
    }

    fn backtrack(nums: &[i32], rem: i32, from: usize) -> bool {
        // We've made a subset with the target sum
        if rem == 0 {
            return true;
        }

        for idx in from..nums.len() {
            // The current number is larger than the remaining sum,
            // thus it cannot be part of this subset
            if nums[idx] > rem {
                continue;
            }

            // Check if we have already tried that number (because the array is sorted)
            // And if the previous try did not work, then this one will not work as well
            if idx > from && nums[idx - 1] == nums[idx] {
                return false;
            }

            // Check if the next number is part of this subset
            if backtrack(nums, rem - nums[idx], idx + 1) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00416_partition_equal_subset_Sum::a416::can_partition;

    #[test]
    fn t_001(){
        let inp = vec![1, 5, 11, 5];



        assert_eq!(true, can_partition(inp) );
    }

    #[test]
    fn t_002(){
        let  inp = vec![1,2,3,5];
        assert_eq!(false,can_partition(inp) );
    }

}