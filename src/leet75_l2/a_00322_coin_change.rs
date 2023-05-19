/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/coin-change/
// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
//
// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
//
// You may assume that you have an infinite number of each kind of coin.
//
//
//
// Example 1:
//
// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
// Example 2:
//
// Input: coins = [2], amount = 3
// Output: -1
// Example 3:
//
// Input: coins = [1], amount = 0
// Output: 0
//
//
// Constraints:
//
// 1 <= coins.length <= 12
// 1 <= coins[i] <= 231 - 1
// 0 <= amount <= 104'


pub mod a322 {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let max_val: usize = amount as usize + 1;
        let mut dp: Vec<i32> = vec![amount + 1; max_val];

        dp[0] = 0;
        for idx in 1..=amount as usize {
            for jdx in 0..coins.len() {
                if coins[jdx] <= idx as i32 {
                    dp[idx] = std::cmp::min(dp[idx], dp[idx - coins[jdx] as usize] + 1)
                }
            }
        }


        if dp[amount as usize] > amount {
            return -1;
        }

        dp[amount as usize]
    }


    pub fn coin_change_faster(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1;amount as usize + 1];
        dp[0] = 0;
        for i in 0..=amount {
            let mut min = std::i32::MAX;
            for value in &coins {
                if i >= *value {
                    let temp = dp[(i-*value) as usize];
                    if temp != -1 && temp+1 < min {
                        min = temp + 1;
                    }
                }
            }
            if min != std::i32::MAX {
                dp[i as usize] = min;
            }
        }
        return dp[amount as usize];
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_l2::a_00322_coin_change::a322::coin_change;

    #[test]
    fn t_000() {
        let inp = vec![1, 2, 5];
        let amount = 11;
        let ans = 3;
        assert_eq!(ans, coin_change(inp, amount));
    }

    #[test]
    fn t_001() {
        let inp = vec![2];
        let amount = 3;
        let ans = -1;

        assert_eq!(ans, coin_change(inp, amount));
    }

    #[test]
    fn t_002() {
        let inp = vec![1];
        let amount = 0;
        let ans = 0;
        assert_eq!(0, coin_change(inp, amount));
    }
}