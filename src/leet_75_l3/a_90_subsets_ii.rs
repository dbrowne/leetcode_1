/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an integer array nums that may contain duplicates, return all possible
// subsets
// (the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in any order.
//
//
//
// Example 1:
//
// Input: nums = [1,2,2]
// Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
// Example 2:
//
// Input: nums = [0]
// Output: [[],[0]]
//
//
// Constraints:
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10


pub  mod  a90 {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &Vec<i32>, start: usize, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            res.push(track.clone());

            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }

                track.push(nums[i]);
                backtrack(nums, i + 1, track, res);
                track.pop();
            }
        }

        let mut res = vec![];
        let mut nums = nums;
        nums.sort_unstable();

        backtrack(&nums, 0, &mut vec![], &mut res);

        res
    }


}


#[cfg(test)]
mod  test{
    use crate::leet_75_l3::a_90_subsets_ii::a90::subsets_with_dup;

    #[test]
    fn t_00(){
        let  inp = vec![1,2,2];
        let  ans = vec![vec![],vec![1],vec![1,2],vec![1,2,2],vec![2],vec![2,2]];
        assert_eq!(ans, subsets_with_dup(inp));
    }
}