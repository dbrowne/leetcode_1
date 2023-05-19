/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/3sum-closest/
// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
//
// Return the sum of the three integers.
//
// You may assume that each input would have exactly one solution.
//
//
//
// Example 1:
//
// Input: nums = [-1,2,1,-4], target = 1
// Output: 2
// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
// Example 2:
//
// Input: nums = [0,0,0], target = 1
// Output: 0
// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//
//
// Constraints:
//
// 3 <= nums.length <= 500
// -1000 <= nums[i] <= 1000
// -104 <= target <= 104

pub  mod  a16{
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {

        let n = nums.len();

        let sorted_nums = {
            let mut temp = nums.clone();
            temp.sort();
            temp
        };

        let mut diff: Option<i32> = None;

        for (i, &i_num) in sorted_nums.iter().enumerate() {
            let (mut low, mut high) = (i+1, n-1);
            while low < high {
                let curr = i_num + sorted_nums[low] + sorted_nums[high];
                if curr == target {
                    return target;
                }

                match diff {
                    None => {diff = Some(target-curr);},
                    Some(d) => {
                        if (target-curr).abs() < d.abs() {
                            diff = Some(target-curr);
                        }

                        if curr < target {
                            low += 1;
                        } else {
                            high -= 1;
                        }
                    }
                }
            }
        }

        target - diff.unwrap_or(0)
    }

    pub fn three_sum_closest_fastest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let lasti = nums.len() - 3;
        let min = nums[0] + nums[1] + nums[2];
        if min >= target {
            return min;
        }
        let max = nums[lasti] + nums[lasti+1] + nums[lasti+2];
        if max <= target {
            return max;
        }
        let mut sum = 0;
        let mut min_diff = i32::MAX;
        let mut i = 0;
        while i <= lasti {
            if i != 0 && nums[i] == nums[i-1] {
                i += 1;
                continue;
            }

            let mut j = i + 1;
            let mut k = lasti + 2;
            while j < k {
                let s = nums[i] + nums[j] + nums[k];
                let diff = s - target;
                let abs_diff = i32::abs(diff);
                if abs_diff < min_diff {
                    min_diff = abs_diff;
                    sum = s;
                }

                match diff {
                    d if d > 0 => k -= 1,
                    d if d < 0 => j += 1,
                    _ => return sum,
                }
            }
            i += 1;
        }

        sum
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_l2::a_0016_3_sum_closest::a16::three_sum_closest;

    #[test]
    fn t_0(){
        assert_eq!(2,three_sum_closest(vec![-1,2,1,-4],1) );
    }
}