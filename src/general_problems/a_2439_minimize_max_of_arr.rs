/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*
You are given a 0-indexed array nums comprising of n non-negative integers.

In one operation, you must:

Choose an integer i such that 1 <= i < n and nums[i] > 0.
Decrease nums[i] by 1.
Increase nums[i - 1] by 1.
Return the minimum possible value of the maximum integer of nums after performing any number of operations.



Example 1:

Input: nums = [3,7,1,6]
Output: 5
Explanation:
One set of optimal operations is as follows:
1. Choose i = 1, and nums becomes [4,6,1,6].
2. Choose i = 3, and nums becomes [4,6,2,5].
3. Choose i = 1, and nums becomes [5,5,2,5].
The maximum integer of nums is 5. It can be shown that the maximum number cannot be less than 5.
Therefore, we return 5.
Example 2:

Input: nums = [10,1]
Output: 10
Explanation:
It is optimal to leave nums as is, and since 10 is the maximum value, we return 10.


Constraints:

n == nums.length
2 <= n <= 105
0 <= nums[i] <= 109
*/

pub  mod  a2439{
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        // Based on C++ solution
        let  mut ans:i64 = 0;
        let  mut prefix_sum: i64 = 0;
        for idx in 0..nums.len() {
           prefix_sum += nums[idx] as i64;
            ans = ans.max((prefix_sum+idx as i64)/(idx as i64 +1));
        }

        ans as i32
    }
}



#[cfg(test)]
mod  test{
    use crate::general_problems::a_2439_minimize_max_of_arr::a2439::minimize_array_value;

    #[test]
    fn t_000(){
        let nums:Vec<i32> = vec![3,7,1,6];
        let  ans:i32 = 5;
        assert_eq!(ans,minimize_array_value(nums) );
    }

    #[test]
    fn t_0001(){
        let  nums:Vec<i32> = vec![10,1];
        let  ans :i32 = 10;
        assert_eq!(ans, minimize_array_value(nums));
    }
}