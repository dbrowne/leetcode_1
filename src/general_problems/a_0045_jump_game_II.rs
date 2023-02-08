/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// 45. Jump Game II
// Medium
// 10.7K
// 373
// company
// Amazon
// company
// Bloomberg
// company
// Apple
// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
//
// Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:
//
// 0 <= j <= nums[i] and
// i + j < n
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].
//
//
//
// Example 1:
//
// Input: nums = [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
// Example 2:
//
// Input: nums = [2,3,0,1,4]
// Output: 2
//
//
// Constraints:
//
// 1 <= nums.length <= 104
// 0 <= nums[i] <= 1000
// Accepted
// 782.1K
// Submissions

pub  mod  a045{
    pub fn jump1(nums: Vec<i32>) -> i32 {
        let  nums_len = nums.len() -1;
        let  mut idx = 0;
        let  mut ans = 0;

        while idx < nums_len {
            ans +=1;
            if idx + nums[idx] as usize >=nums_len {
                return ans;
            }
            let  skip = (nums_len -idx).min(nums[idx] as usize,);
            let  mut max_val = 0;
            let  mut t_idx = 0;

            let  mut jdx = 1;

            while jdx <=skip {
                if  max_val < jdx as i32 + nums[idx+jdx] {
                    max_val = jdx as i32 + nums[idx + jdx];
                    t_idx = jdx;
                }
                jdx +=1;

            }
            idx += t_idx;
        }


       ans
    }


    pub  fn  jump(nums: Vec<i32>) ->i32{
        let  mut ans = 0;
        let  nums_len = nums.len();
        let  mut current_pos :i32 = 0;
        let  mut cur_jump = 0;

        for idx in 0..(nums_len -1){
           cur_jump = cur_jump.max(idx as i32 +nums[idx]) ;
            if idx as i32 == current_pos {
                ans +=1;
                current_pos = cur_jump;
            }
        }
        ans
    }

    pub fn jump2(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = (0,0);
        while dp.1 < nums.len() - 1 {
            let mut next = vec![];
            for i in dp.0..dp.1 + 1{
                next.push(i + nums[i] as usize);
            }
            dp = (dp.1, *next.iter().max().unwrap());
            res += 1;
        }
        return res
    }

}



#[cfg(test)]
mod  test{
    use crate::general_problems::a_0045_jump_game_II::a045::jump;

    #[test]
    fn t_01(){
        let  inp:Vec<i32> = vec![2,3,1,1,4];
        let and = 2;
        assert_eq!(jump(inp),2)
    }

    #[test]
    fn t_02(){
        let  inp:Vec<i32> = vec![2,3,0,1,4];
        let  ans = 2;
        assert_eq!(ans, jump(inp));

    }
}