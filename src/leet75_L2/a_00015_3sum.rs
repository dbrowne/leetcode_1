/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/3sum/
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
//
//
// Example 1:
//
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
// Example 2:
//
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
// Example 3:
//
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
//
//
// Constraints:
//
// 3 <= nums.length <= 3000
// -105 <= nums[i] <= 105


pub mod a15 {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums.clone();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        res
    }
}


#[cfg(test)]
mod test {
    use crate::leet75_L2::a_00015_3sum::a15::three_sum;

    #[test]
    fn t_0001() {
        let inp = vec![-1, 0, 1, 2, -1, -4];
        let ans = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(ans, three_sum(inp));
    }

    #[test]
    fn t_0002() {
        let inp = vec![0, 1, 1];
        let ans: Vec<Vec<i32>> = vec![];

        assert_eq!(ans, three_sum(inp));
    }

    #[test]
    fn t_003() {
        let inp = vec![0, 0, 0];
        let ans = vec![vec![0, 0, 0]];
        assert_eq!(ans, three_sum(inp));
    }
}