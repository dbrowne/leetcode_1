/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/number-of-1-bits/
//
// Write a function that takes an unsigned integer and returns the number of '1' bits it
// has (also known as the Hamming weight).
//
// Note:
//
// Note that in some languages, such as Java, there is no unsigned integer type.
// In this case, the input will be given as a signed integer type.
// It should not affect your implementation, as the integer's internal binary representation
// is the same, whether it is signed or unsigned.
// In Java, the compiler represents the signed integers using 2's complement notation.
// Therefore, in Example 3, the input represents the signed integer. -3.
//
//
// Example 1:
//
// Input: n = 00000000000000000000000000001011
// Output: 3
// Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
// Example 2:
//
// Input: n = 00000000000000000000000010000000
// Output: 1
// Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
// Example 3:
//
// Input: n = 11111111111111111111111111111101
// Output: 31
// Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
//
//
// Constraints:
//
// The input must be a binary string of length 32.
//
//
// Follow up: If this function is called many times, how would you optimize it?

pub  mod  a191{
    pub  fn  hamminWeight(n: u32) ->i32{
        n.count_ones() as i32
    }
}


#[cfg(test)]
mod  test{
    use crate::leet_75_l3::a_191_number_of_1_bits::a191::hamminWeight;

    #[test]
    fn t_001(){
        let  inp:u32 = 0b00000000000000000000000000001011;
        let  ans = 3;
        assert_eq!(ans, hamminWeight(inp));

    }

    #[test]
    fn t_002(){
        let  inp:u32 = 0b00000000000000000000000010000000;
        let  ans = 1;
        assert_eq!(ans, hamminWeight(inp));
    }

    #[test]
    fn t_003(){
        let  inp:u32=0b11111111111111111111111111111101;
        let  ans =31;

        assert_eq!(ans, hamminWeight(inp));
    }
}