/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
//
// The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
//
// Return the quotient after dividing dividend by divisor.
//
// Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−231, 231 − 1]. For this problem, if the quotient is strictly greater than 231 - 1, then return 231 - 1, and if the quotient is strictly less than -231, then return -231.
//
//
//
// Example 1:
//
// Input: dividend = 10, divisor = 3
// Output: 3
// Explanation: 10/3 = 3.33333.. which is truncated to 3.
// Example 2:
//
// Input: dividend = 7, divisor = -3
// Output: -2
// Explanation: 7/-3 = -2.33333.. which is truncated to -2.
//
//
// Constraints:
//
// -231 <= dividend, divisor <= 231 - 1
// divisor != 0



pub mod a29 {

    pub  fn  divide(dividend: i32, divisor: i32) -> i32{

        let  dividend:i64 = dividend as i64;
        let divisor:i64 = divisor as i64;
        let  mut sign:i64 = 1;
        if dividend < 0 {
            sign *= -1;
        }
        if divisor < 0 {
            sign *= -1;
        }

        let  mut dividend = dividend.abs();
        let  mut divisor = divisor.abs();
        let  mut quotient:i64 = 0;
        while dividend >= divisor {
            divisor += divisor;
            quotient += 2;
        }

        if divisor > dividend {
            quotient -= 2;
        }
        quotient *= sign;

        if  quotient  >i32::max_value() as i64{
            return i32::max_value();
        }

        if  quotient < i32::min_value() as i64 {
            return i32::min_value();
        }

        quotient as i32
    }
}



#[cfg(test)]
mod  test{
    #[test]
    fn t_001(){
        let  dividend = 10;
        let  divisor = 3;
        let  expected = 3;
        let  result = super::a29::divide(dividend, divisor);
        assert_eq!(result, expected);
    }

    #[test]
    fn t_002(){
        let  dividend = 7;
        let  divisor = -3;
        let  expected = -2;
        let  result = super::a29::divide(dividend, divisor);
        assert_eq!(result, expected);
    }
}