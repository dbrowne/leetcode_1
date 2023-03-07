/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//
// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// nums[a] + nums[b] + nums[c] + nums[d] == target
// You may return the answer in any order.
//
//
//
// Example 1:
//
// Input: nums = [1,0,-1,0,-2,2], target = 0
// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
// Example 2:
//
// Input: nums = [2,2,2,2,2], target = 8
// Output: [[2,2,2,2]]
//
//
// Constraints:
//
// 1 <= nums.length <= 200
// -109 <= nums[i] <= 109
// -109 <= target <= 109

pub mod a18 {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        const THREE: usize = 3;
        const TWO: usize = 2;
        const ONE: usize = 1;
        let num_len: usize = nums.len();

        if num_len < 4 {
            return vec![];
        }

        let mut nums: Vec<i32> = nums.clone();
        nums.sort();

        let mut ans: Vec<Vec<i32>> = vec![];

        for i in 0..(num_len - THREE) {
            if i > 0 && nums[i] == nums[i - ONE] {
                continue;
            }
            for j in (i + 1)..(num_len - TWO) {
                if j > i + 1 && nums[j] == nums[j - ONE] {
                    continue;
                }

                let target: i64 = target as i64 - nums[i] as i64 - nums[j] as i64;

                let mut left: usize = j + ONE;

                let mut right: usize = num_len - ONE;

                while left < right {

                    if (nums[left] as i64 + nums[right] as i64 )== target as i64{

                        ans.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        left += ONE;
                        right -= ONE;

                        while left < right && nums[left] == nums[left - ONE] {
                            left += ONE;
                        }

                        while left < right && nums[right] == nums[right + ONE] {
                            right -= ONE;
                        }
                    }
                    if (nums[left] as i64+ nums[right] as i64) > target as i64{
                        right -= ONE;
                    }
                    if (nums[left] as i64 + nums[right] as i64 )< target  as i64{
                        left += ONE;
                    }
                }
            }
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0018_4sum::a18::four_sum;

    #[test]
    fn t_001() {
        let inp: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let ans: Vec<Vec<i32>> = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];

        assert_eq!(ans, four_sum(inp, target));
    }


    #[test]
    fn t_002(){
        let  inp :Vec<i32> = vec![1000000000,1000000000,1000000000,1000000000];
        let  target = -294967296;
        let  ans:Vec<Vec<i32>> = vec![];

        assert_eq!(ans, four_sum(inp,target));


    }
}