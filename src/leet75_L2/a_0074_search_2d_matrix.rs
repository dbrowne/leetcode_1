/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/search-a-2d-matrix/
// You are given an m x n integer matrix matrix with the following two properties:
//
// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the previous row.
// Given an integer target, return true if target is in matrix or false otherwise.
//
// You must write a solution in O(log(m * n)) time complexity.
//
//
//
// Example 1:
//
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
// Output: true
// Example 2:
//
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
// Output: false
//
//
// Constraints:
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 100
// -104 <= matrix[i][j], target <= 104


pub  mod  a74{
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let  row_size = matrix.len();
        for rw in 0..row_size  {
            if  target <= *matrix[rw].last().unwrap() && target >=matrix[rw][0]{
                let  r = matrix[rw].binary_search(&target);
                return match r {
                    Ok(..) => true,
                    _ => false,
                };
            }
        }
        false
    }
}

#[cfg(test)]
mod  test {
    use crate::leet75_L2::a_0074_search_2d_matrix::a74::search_matrix;

    #[test]
    fn t_001(){
        let  tv:Vec<Vec<i32>>= vec![vec![1,2,3,4,5],vec![6,7,8,9,10],vec![11,12,13,14,15]];

        assert_eq!(true,search_matrix(tv,5));
    }


    #[test]
    fn t_002(){
        let  tv:Vec<Vec<i32>>= vec![vec![1,2,3,4,5],vec![6,7,8,9,10],vec![11,12,13,14,15]];

        assert_eq!(false,search_matrix(tv,16));
    }
}