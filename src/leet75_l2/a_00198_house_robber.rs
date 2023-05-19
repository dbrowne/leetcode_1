/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/house-robber/?
//
// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
//
// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
// Total amount you can rob = 1 + 3 = 4.
// Example 2:
//
// Input: nums = [2,7,9,3,1]
// Output: 12
// Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
// Total amount you can rob = 2 + 9 + 1 = 12.
//
//
// Constraints:
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 400

// Imagine we are standing at house i. We can rob the house, but then we can't rob the house before it, so the maximum total we can have is the maximum total we can have at house i-2, plus what we get at house i. Or we can keep whatever we can get up to house i-1, and avoid robbing house i. The base case is that without any houses, there is no money to rob, so the initial state for the DP is (0,0).
//
// I use into_iter to take ownership of nums and iterate over the numbers directly, and not complicate things with references. Then a fold to propagate the state according to the logic above.

mod  a198{
    pub  fn rob(nums:Vec<i32>)->i32{
        nums.into_iter().fold((0, 0), |(pp, p), curr| (p, p.max(curr + pp))).1
    }


    pub fn rob_faster(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        if nums.len() == 1 {
            return nums[0];
        }

        let mut one = nums[0];
        let mut two = max(nums[0], nums[1]);

        for i in 2..(nums.len()) {
            let mut tmp = max(one + nums[i], two);
            one = two;
            two = tmp;
        }

        two
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_l2::a_00198_house_robber::a198::rob;

    #[test]
    fn t_0000(){
        assert_eq!(0, rob(vec![0]));
    }
}