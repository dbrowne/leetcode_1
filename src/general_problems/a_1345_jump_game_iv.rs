/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of integers arr, you are initially positioned at the first index of the array.
//
// In one step you can jump from index i to index:
//
// i + 1 where: i + 1 < arr.length.
// i - 1 where: i - 1 >= 0.
// j where: arr[i] == arr[j] and i != j.
// Return the minimum number of steps to reach the last index of the array.
//
// Notice that you can not jump outside of the array at any time.
//
//
//
// Example 1:
//
// Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
// Output: 3
// Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
// Example 2:
//
// Input: arr = [7]
// Output: 0
// Explanation: Start index is the last index. You do not need to jump.
// Example 3:
//
// Input: arr = [7,6,9,6,9,6,9,7]
// Output: 1
// Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
//
//
// Constraints:
//
// 1 <= arr.length <= 5 * 104
// -108 <= arr[i] <= 108

pub mod a1345 {
    use std::collections::HashMap;

    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let arr_l:usize = arr.len();
        if arr_l <= 1 {
            return 0;
        }
        if arr_l == 2 || arr[0] == arr[arr_l - 1] {
            return 1;
        }

        let mut idx: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..arr_l {
            idx.entry(arr[i]).or_default()
               .push(i);
        }
        let mut missed:Vec<bool> = vec![true; arr_l];
        let mut ans:i32 = 0;
        let mut curr:Vec<usize> = vec![];
        let mut nxt:Vec<usize> = vec![];

        curr.push(0);
        while !curr.is_empty() {
            for i in curr.drain(..) {
                if i == arr_l - 1 {
                    return ans;
                }
                if i > 0 && missed[i - 1] {
                    nxt.push(i - 1);
                    missed[i - 1] = false;
                }

                if i + 1 < arr_l && missed[i + 1] {
                    nxt.push(i + 1);
                    missed[i + 1] = false;
                }

                if let Some(v) = idx.remove(&arr[i]) {
                    for ii in v {
                        if missed[ii] {
                            nxt.push(ii);
                            missed[ii] = false;
                        }
                    }
                }
            }
            std::mem::swap(&mut curr, &mut nxt);
            ans += 1;
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_1345_jump_game_iv::a1345::min_jumps;

    #[test]
    fn t_001() {
        let inp: Vec<i32> = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        let ans: i32 = 3;

        assert_eq!(ans, min_jumps(inp));
    }


    #[test]
    fn t_002() {
        let inp: Vec<i32> = vec![7];
        let ans: i32 = 0;

        assert_eq!(ans, min_jumps(inp));
    }

    #[test]
    fn t_003() {
        let inp: Vec<i32> = vec![7, 6, 9, 6, 9, 6, 9, 7];
        let ans: i32 = 1;
        assert_eq!(ans, min_jumps(inp));
    }
}