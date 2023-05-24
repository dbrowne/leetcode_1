/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas.
// The guards have gone and will come back in h hours.
//
// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of
// bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of
// them instead and will not eat any more bananas during this hour.
//
// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
//
// Return the minimum integer k such that she can eat all the bananas within h hours.
//
//
//
// Example 1:
//
// Input: piles = [3,6,7,11], h = 8
// Output: 4
// Example 2:
//
// Input: piles = [30,11,23,4,20], h = 5
// Output: 30
// Example 3:
//
// Input: piles = [30,11,23,4,20], h = 6
// Output: 23
//
//
// Constraints:
//
// 1 <= piles.length <= 104
// piles.length <= h <= 109
// 1 <= piles[i] <= 109

pub mod a875 {

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        //based on c++ binary search solution
        if piles.len() == 1 {
            let mut x: i64 = piles[0] as i64 / h as i64;
            if piles[0] as i64 % h as i64 != 0 {
                x += 1;
            }
            return x as i32;
        }

        let mut right: i64 = *piles.iter().max().unwrap() as i64;
        let mut left: i64 = 0;

        while left < right {
            let mid: i64 = (left + right) / 2;
            let mut hours_spent: i64 = 0;
            for p in &piles {
                let p64: i64 = *p as i64;
                hours_spent += p64 / mid;
                if p64 % mid != 0 {
                    hours_spent += 1;
                }
            }
            if hours_spent <= h as i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0875_koko_eating_bananas::a875::min_eating_speed;

    #[test]
    fn t_001() {
        let piles: Vec<i32> = vec![3, 6, 7, 11];
        let h = 8;
        let ans = 4;
        assert_eq!(ans, min_eating_speed(piles, h));
    }

    #[test]
    fn t_002() {
        let piles: Vec<i32> = vec![30, 11, 23, 4, 20];
        let h = 5;
        let ans = 30;

        assert_eq!(ans, min_eating_speed(piles, h));
    }

    #[test]
    fn t_003() {
        let piles: Vec<i32> = vec![30, 11, 23, 4, 20];
        let h = 6;
        let ans = 23;
        assert_eq!(ans, min_eating_speed(piles, h));
    }

    #[test]
    fn t_004() {
        let piles: Vec<i32> = vec![312884470];
        let h = 968709470;
        let ans = 1;
        assert_eq!(ans, min_eating_speed(piles, h));
    }
}

