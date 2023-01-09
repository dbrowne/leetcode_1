/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/rotting-oranges
// You are given an m x n grid where each cell can have one of three values:
//
// 0 representing an empty cell,
// 1 representing a fresh orange, or
// 2 representing a rotten orange.
// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
//
// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
//
//
//
// Example 1:
//
//
// Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
// Output: 4
// Example 2:
//
// Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
// Output: -1
// Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
// Example 3:
//
// Input: grid = [[0,2]]
// Output: 0
// Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 10
// grid[i][j] is 0, 1, or 2.

pub mod a994 {
    use std::collections::VecDeque;
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid:Vec<Vec<i32>> =grid.clone();
        let mut q : VecDeque<(usize, usize)> = VecDeque::new();
        let mut num_fresh = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                match grid[r][c]{
                    1 => num_fresh += 1,
                    2 => {
                        q.push_back((r, c));
                    },
                    _ => {}
                }
            }
        }
        if num_fresh == 0 {
            return 0;
        }
        let mut num_minutes = -1;
        let mut dirs : [(i32, i32); 4] = [(-1, 0), (1 , 0), (0, -1), (0, 1)];
        while !q.is_empty() {
            let level_size = q.len();
            for i in 0..level_size {
                if let Some((r, c)) = q.pop_front() {
                    for (dr, dc) in dirs {
                        let (nr, nc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
                        if 0 <= nr &&
                            nr < grid.len() &&
                            0 <= nc &&
                            nc < grid[0].len() &&
                            grid[nr][nc] == 1 {
                            grid[nr][nc] = 2;
                            q.push_back((nr, nc));
                            num_fresh -= 1;
                        }
                    }
                }
            }
            num_minutes += 1;
        }
        if num_fresh == 0 {num_minutes } else {-1}
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00994_rotting_oranges::a994::oranges_rotting;

    #[test]
    fn t_000(){
        let  inp:Vec<Vec<i32>> = vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]];
        assert_eq!(-1,oranges_rotting(inp) );
    }
}