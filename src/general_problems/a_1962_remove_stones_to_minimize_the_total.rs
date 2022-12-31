/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// You are given a 0-indexed integer array piles, where piles[i] represents the number of stones in the ith pile, and an integer k. You should apply the following operation exactly k times:
//
// Choose any piles[i] and remove floor(piles[i] / 2) stones from it.
// Notice that you can apply the operation on the same pile more than once.
//
// Return the minimum possible total number of stones remaining after applying the k operations.
//
// floor(x) is the greatest integer that is smaller than or equal to x (i.e., rounds x down).
//
//
//
// Example 1:
//
// Input: piles = [5,4,9], k = 2
// Output: 12
// Explanation: Steps of a possible scenario are:
// - Apply the operation on pile 2. The resulting piles are [5,4,5].
// - Apply the operation on pile 0. The resulting piles are [3,4,5].
// The total number of stones in [3,4,5] is 12.
// Example 2:
//
// Input: piles = [4,3,6,7], k = 3
// Output: 12
// Explanation: Steps of a possible scenario are:
// - Apply the operation on pile 2. The resulting piles are [4,3,3,7].
// - Apply the operation on pile 3. The resulting piles are [4,3,3,4].
// - Apply the operation on pile 0. The resulting piles are [2,3,3,4].
// The total number of stones in [2,3,3,4] is 12.
//
//
// Constraints:
//
// 1 <= piles.length <= 105
// 1 <= piles[i] <= 104
// 1 <= k <= 105

pub mod a1962 {
    use std::collections::BinaryHeap;
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut total_sum =0;
        let  t_vec = piles.clone();

        let  mut stone_heap = BinaryHeap::from(piles);
        for i  in t_vec {
            total_sum +=i;

        }

        for i in 0..k  {
            let  curr = stone_heap.pop().unwrap();
            let  remove = curr/2;
            total_sum -= remove;
            stone_heap.push(curr - remove);


        }

        total_sum
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_1962_remove_stones_to_minimize_the_total::a1962::min_stone_sum;

    #[test]
    fn test_00() {
        let inp = vec![5, 4, 9];
        let ans = 12;
        assert_eq!(ans, min_stone_sum(inp, 2));
    }

    #[test]
    fn test_01() {
        let inp: Vec<i32> = vec![4, 3, 6, 7];
        let ans = 12;
        assert_eq!(ans, min_stone_sum(inp, 3));
    }
}