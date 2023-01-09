/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/pacific-atlantic-water-flow/
// There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
//
// The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
//
// The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
//
// Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
//
//
//
// Example 1:
//
//
// Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
// Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
// Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
// [0,4]: [0,4] -> Pacific Ocean
// [0,4] -> Atlantic Ocean
// [1,3]: [1,3] -> [0,3] -> Pacific Ocean
// [1,3] -> [1,4] -> Atlantic Ocean
// [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
// [1,4] -> Atlantic Ocean
// [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
// [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
// [3,0]: [3,0] -> Pacific Ocean
// [3,0] -> [4,0] -> Atlantic Ocean
// [3,1]: [3,1] -> [3,0] -> Pacific Ocean
// [3,1] -> [4,1] -> Atlantic Ocean
// [4,0]: [4,0] -> Pacific Ocean
// [4,0] -> Atlantic Ocean
// Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
// Example 2:
//
// Input: heights = [[1]]
// Output: [[0,0]]
// Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
//
//
// Constraints:
//
// m == heights.length
// n == heights[r].length
// 1 <= m, n <= 200
// 0 <= heights[r][c] <= 105

pub  mod  a0417{
    use std::collections::VecDeque;

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let mut res = vec![];
        let rows = heights.len();
        let cols = heights.first().map_or(0, |v| v.len());

        if rows == 0 {
            return res;
        }

        let mut path = (0..rows).map(|_| vec![0u8; cols]).collect::<Vec<Vec<u8>>>();
        let mut points = VecDeque::new();

        (0..rows).for_each(|r| {
            points.push_back((r as i32, 0, 1));
            path[r][0] = 1;

            points.push_back((r as i32, (cols - 1) as i32, 2));
            path[r][cols - 1] = 2;
        });

        (0..cols).for_each(|c| {
            points.push_back((0, c as i32, 1));
            path[0][c] = 1;

            points.push_back(((rows - 1) as i32, c as i32, 2));
            path[rows - 1][c] = 2;
        });

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((x, y, ocean)) = points.pop_front() {

            path[x as usize][y as usize] |= ocean;

            for &(xd, yd) in directions.iter() {
                let (x_new, y_new) = (x + xd, y + yd);

                // check out of bounds
                if x_new < 0 || x_new > (rows - 1) as i32 || y_new < 0 || y_new > (cols - 1) as i32 {
                    continue;
                }

                // check visited
                let new_ocean = path[x_new as usize][y_new as usize];
                if new_ocean == ocean || new_ocean == 3 {
                    continue;
                }

                // check heights
                if heights[x_new as usize][y_new as usize] < heights[x as usize][y as usize] {
                    continue;
                }

                points.push_back((x_new, y_new, ocean));
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if path[r][c] == 3 {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }

        res
    }



    ///// Alternate solution:
    use std::cmp;

    struct Solution{}

    struct Coor {
        row: i32,
        col: i32,
    }

    impl Coor {
        pub fn new(row: i32, col: i32) -> Coor {
            Coor { row: row, col: col }
        }

        pub fn Valid(&self, width: usize, height: usize) -> bool {
            return self.row >= 0 && self.col >= 0
                && self.Col() < width && self.Row() < height;
        }

        pub fn Row(&self) -> usize {
            return self.row as usize;
        }

        pub fn Col(&self) -> usize {
            return self.col as usize;
        }
    }

    impl Solution {
        pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let height = heights.len();
            let width = heights[0].len();

            let pac: Vec<Vec<i32>>;
            {
                let mut pacInit: Vec<Coor> = Vec::new();
                for col in 0..width {
                    pacInit.push(Coor::new(0, col as i32));
                }
                for row in 1..height {
                    pacInit.push(Coor::new(row as i32, 0));
                }
                pac = Solution::BFS(pacInit, &heights);
            }

            let at: Vec<Vec<i32>>;
            {
                let mut atInit: Vec<Coor> = Vec::new();
                for col in 0..width {
                    atInit.push(Coor::new(height as i32 - 1, col as i32));
                }
                for row in 0..height {
                    atInit.push(Coor::new(row as i32, width as i32 - 1));
                }
                at = Solution::BFS(atInit, &heights);
            }

            let mut result: Vec<Vec<i32>> = Vec::new();
            for r in 0..height {
                for c in 0..width {
                    if pac[r][c] == 1 && at[r][c] == 1 {
                        result.push(vec![r as i32, c as i32]);
                    }
                }
            }
            return result;
        }

        pub fn BFS(init: Vec<Coor>, heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let height = heights.len();
            let width = heights[0].len();
            let mut grid: Vec<Vec<i32>> = vec![vec![0; width]; height];

            for c in &init {
                grid[c.Row()][c.Col()] = 1;
            }

            let mut level = init;
            while !level.is_empty() {
                let mut newLevel: Vec<Coor> = Vec::new();
                for c in level {
                    let newCoor: [Coor; 4] = [
                        Coor::new(c.row - 1, c.col),
                        Coor::new(c.row + 1, c.col),
                        Coor::new(c.row, c.col + 1),
                        Coor::new(c.row, c.col - 1)];

                    for nc in newCoor {
                        if !nc.Valid(width, height) {
                            continue;
                        }
                        // if grid[nc.Row()][nc.Col()] != -1 {
                        //     continue;
                        // }
                        let mut newVal = 0;
                        if heights[nc.Row()][nc.Col()]
                            >= heights[c.Row()][c.Col()] {
                            newVal = 1;
                        }

                        if grid[nc.Row()][nc.Col()] == 0 && newVal == 1 {
                            grid[nc.Row()][nc.Col()] = 1;
                            newLevel.push(nc);
                        }
                    }
                }
                level = newLevel;
            }
            return grid;
        }
    }
}


#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00417_pacific_atlantic_water_flow::a0417::pacific_atlantic;

    #[test]
    fn t_000(){

        let  inp:Vec<Vec<i32>> = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],
                          vec![6,7,1,4,5],vec![5,1,1,2,4]];
        let  ans:Vec<Vec<i32>> = vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]];
        assert_eq!(ans,pacific_atlantic(inp));
    }
}
