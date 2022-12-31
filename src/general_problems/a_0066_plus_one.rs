/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/plus-one/
//
// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
//
// Increment the large integer by one and return the resulting array of digits.
//
//
//
// Example 1:
//
// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Incrementing by one gives 123 + 1 = 124.
// Thus, the result should be [1,2,4].
// Example 2:
//
// Input: digits = [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.
// Incrementing by one gives 4321 + 1 = 4322.
// Thus, the result should be [4,3,2,2].
// Example 3:
//
// Input: digits = [9]
// Output: [1,0]
// Explanation: The array represents the integer 9.
// Incrementing by one gives 9 + 1 = 10.
// Thus, the result should be [1,0].
//
//
// Constraints:
//
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits does not contain any leading 0's.


pub mod a66 {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let d_len = digits.len();
        let mut ans: Vec<i32> = digits.clone();
        for idx in (0..d_len).rev() {
            if ans[idx] == 9 {
                ans[idx] = 0;
            } else {
                ans[idx] += 1;
                return ans;
            }
        }
        [vec![1],ans].concat()


    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0066_plus_one::a66::plus_one;

    #[test]
    fn test_000() {
        assert_eq!(vec![1,2,4], plus_one(vec![1,2,3]))
    }
    #[test]
    fn test_001(){
        assert_eq!(vec![1,0],plus_one(vec![9]) );
    }

}