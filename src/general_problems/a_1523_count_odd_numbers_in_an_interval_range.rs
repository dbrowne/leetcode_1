/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).
//
//
//
// Example 1:
//
// Input: low = 3, high = 7
// Output: 3
// Explanation: The odd numbers between 3 and 7 are [3,5,7].
// Example 2:
//
// Input: low = 8, high = 10
// Output: 1
// Explanation: The odd numbers between 8 and 10 are [9].
//
//
// Constraints:
//
// 0 <= low <= high <= 10^9

pub  mod  a_1523{


    pub fn count_odds(low: i32, high: i32) -> i32 {
        let  mut ans:i32 =0;
        let  mut new_low = low;

        if  low == high && low%2 !=0 {
            return 1;
        }

        if  low == high && low%2 ==0{
            return 0;
        }

        if  low%2 ==0{
            new_low = low +1;
        }

        if  new_low <= high{
            ans = (high - new_low)/2 +1;
        }
        ans
    }

}

#[cfg(test)]
mod test {
    use crate::general_problems::a_1523_count_odd_numbers_in_an_interval_range::a_1523::count_odds;

    #[test]
    fn t_001(){

        let low = 3;
        let high = 7;
        let ans = 3;

        assert_eq!(ans,count_odds(low,high))


    }


    #[test]
    fn t_002(){
        let low = 8;
        let  high =10;
        let  ans = 1;
        assert_eq!(ans, count_odds(low, high));
    }

    #[test]
    fn t_003(){
        let  low = 17;
        let  high = 17;
        let  ans =1 ;
        assert_eq!(ans, count_odds(low,high));
    }


}