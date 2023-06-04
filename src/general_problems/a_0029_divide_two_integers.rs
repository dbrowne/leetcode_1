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
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }

        let dividend: i64 = dividend as i64;
        let divisor: i64 = divisor as i64;
        let mut sign: i64 = 1;
        if dividend < 0 {
            sign = -sign;
        }
        if divisor < 0 {
            sign = -sign;
        }

        let mut dividend = dividend.abs();
        let mut divisor = divisor.abs();
        let mut quotient = 0;

        let mut remainder: i64 = 1;
        while remainder != 0 {
            let mut tmp;
            (tmp, remainder) = part_div(dividend, divisor);
            dividend = remainder;
            quotient += tmp;
        }


        if sign < 0 {
            quotient = -quotient;
        }
        quotient
    }

    pub fn part_div(dividend: i64, divisor: i64) -> (i32, i64) {
        if (dividend - divisor) == 0 {
            return (1, 0 as i64);
        } else if dividend < divisor {
            return (0, 0 as i64);
        }

        let mut ctr = 1;
        let mut tmp: i64 = divisor.clone();
        let mut remainder: i64 = 0;
        loop {
            tmp += tmp;
            if tmp > dividend {
                break;
            }
            if ctr == 1 {
                ctr = 2;
            } else {
                ctr += ctr;
            }
            remainder = dividend - tmp;
        }

        (ctr, remainder)
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0029_divide_two_integers::a29::{divide, part_div};

    #[test]
    fn t_001() {
        let dividend = 10;
        let divisor = 3;
        let expected = 3;
        let result = super::a29::divide(dividend, divisor);
        assert_eq!(result, expected);
    }

    #[test]
    fn t_002() {
        let dividend = 7;
        let divisor = -3;
        let expected = -2;
        let result = super::a29::divide(dividend, divisor);
        assert_eq!(result, expected);
    }

    #[test]
    fn t_003() {
        let dividend: i64 = 117;
        let divisor: i64 = 8;
        assert_eq!((8, 53), part_div(dividend, divisor));
    }

    #[test]
    fn t_004() {
        let dividend: i64 = 53;
        let divisor: i64 = 8;
        assert_eq!((4, 21), part_div(dividend, divisor));
    }

    #[test]
    fn t_005() {
        let dividend: i64 = 21;
        let divisor: i64 = 8;
        assert_eq!((2, 5), part_div(dividend, divisor));
    }

    #[test]
    fn t_006() {
        let d: i64 = 5;
        let dd: i64 = 8;
        assert_eq!((0, 0), part_div(d, dd));
    }

    #[test]
    fn t_007() {
        let big_d = 2147483645;
        let divisor = 117557;
        assert_eq!(18267, divide(big_d, divisor));
    }
}