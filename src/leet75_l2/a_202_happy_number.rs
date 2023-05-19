/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Write an algorithm to determine if a number n is happy.
//
// A happy number is a number defined by the following process:
//
// Starting with any positive integer, replace the number by the sum of the squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
// Return true if n is a happy number, and false if not.
//
//
//
// Example 1:
//
// Input: n = 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1
// Example 2:
//
// Input: n = 2
// Output: false
//
//
// Constraints:
//
// 1 <= n <= 231 - 1

pub  mod p1{

    use std::collections::HashMap;

    pub fn is_happy(n: i32) -> bool {
        let mut cycle: HashMap<i32, i32> = HashMap::new();
        let  mut ans = n;

        while ans !=1 {

            ans = make_sum_square(ans);
            if  cycle.contains_key(&ans){
                return false;
            }
            cycle.insert(ans, 1);

        }



        true
    }

    pub  fn  make_sum_square(inp:i32) ->i32{

        let  mut xx = inp;
        let mut sm:i32 = 0;

        while xx >=10 {
            sm += (xx%10).pow(2);

            xx = xx/10;
        }

        sm += xx.pow(2);

        sm
    }
}
#[cfg(test)]
mod test{
    use crate::leet75_l2::a_202_happy_number::p1::{is_happy, make_sum_square};

    #[test]
    fn test_00(){
        let  ans =82;
        assert_eq!(ans,make_sum_square(19));
    }
    #[test]
    fn test_000(){
        assert_eq!(1,make_sum_square(100))
    }

    #[test]
    fn test_01(){
        assert_eq!(true,is_happy(19) );
    }

}