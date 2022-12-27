/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// You are given an array of integers stones where stones[i] is the weight of the ith stone.
//
// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
//
// If x == y, both stones are destroyed, and
// If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
// At the end of the game, there is at most one stone left.
//
// Return the weight of the last remaining stone. If there are no stones left, return 0.
//
//
//
// Example 1:
//
// Input: stones = [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
// Example 2:
//
// Input: stones = [1]
// Output: 1
//
//
// Constraints:
//
// 1 <= stones.length <= 30
// 1 <= stones[i] <= 1000

pub  mod xx1{
    use std::collections::BinaryHeap;


    pub  fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let  mut heap:BinaryHeap<i32> = BinaryHeap::new();
        for stone  in stones.iter() {
            heap.push(*stone);
        }

        while heap.len() >1 {
            let  stone1 = heap.pop().unwrap();
            let  stone2 = heap.pop().unwrap();
            if  stone1 != stone2{
                heap.push( stone1 - stone2);
            }

        }
        if  !heap.is_empty() {
            return heap.pop().unwrap();
        }
        0
    }

    pub  fn  last_stone_weight_fast(stones: Vec<i32>) -> i32{
        let  mut max_heap = BinaryHeap::from(stones);

        loop {
            match (max_heap.pop(), max_heap.pop()) {
                (Some(y), Some(x)) =>max_heap.push(y-x),
                (Some(y),None) => return y,
                (None,_) => return 0,
            }
        }

    }

}


#[cfg(test)]
mod test{
    use crate::leetweektwo::a_1046_last_stone_weight::xx1::last_stone_weight;
    #[test]
    fn test_00(){
        assert_eq!(1,last_stone_weight(vec![2,7,4,1,8,1]))
    }

    #[test]
    fn test_01(){
        assert_eq!(1,last_stone_weight(vec![1]) );
    }

}