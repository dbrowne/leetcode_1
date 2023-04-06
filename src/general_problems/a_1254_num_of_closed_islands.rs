/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*
Given a 2D grid consists of 0s (land) and 1s (water).
An island is a maximal 4-directionally connected group of 0s and a closed island is an
island totally (all left, top, right, bottom) surrounded by 1s.

Return the number of closed islands.



Example 1:



Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
Output: 2
Explanation:
Islands in gray are closed because they are completely surrounded by water (group of 1s).
Example 2:



Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
Output: 1
Example 3:

Input: grid = [[1,1,1,1,1,1,1],
               [1,0,0,0,0,0,1],
               [1,0,1,1,1,0,1],
               [1,0,1,0,1,0,1],
               [1,0,1,1,1,0,1],
               [1,0,0,0,0,0,1],
               [1,1,1,1,1,1,1]]
Output: 2


Constraints:

1 <= grid.length, grid[0].length <= 100
0 <= grid[i][j] <=1

*/

pub mod a1254 {
    const VISITED: i32 = 2;
    const ISLAND: i32 = 0;

    fn visit_cells(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) {
        if grid[row][col] != ISLAND {
            return;
        }
        grid[row][col] = VISITED;
        if col < grid[row].len() - 1 {
            visit_cells(grid, row, col + 1);
        }
        if row < grid.len() - 1 {
            visit_cells(grid, row + 1, col);
        }
        if row != 0 {
            visit_cells(grid, row - 1, col);
        }
        if col != 0 {
            visit_cells(grid, row, col - 1);
        }
    }


    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut cloned_grid = grid.clone();

        // mark cells connected to the border
        for col in 0..cloned_grid[0].len() {
            if cloned_grid[0][col] == ISLAND {
                visit_cells(&mut cloned_grid, 0, col);
            }
        }
        for col in 0..cloned_grid[cloned_grid.len() - 1].len() {
            if cloned_grid[cloned_grid.len() - 1][col] == ISLAND {
                visit_cells(&mut cloned_grid, grid.len() - 1, col);  //eliminate E0502 a bit of a
                                                                          //cheat but I don't like mutating my inputs
                                                                         // and

            }
        }
        for row in 1..cloned_grid.len() - 1 {
            if cloned_grid[row][0] == ISLAND {
                visit_cells(&mut cloned_grid, row, 0);
            }

            if cloned_grid[row][cloned_grid[row].len() - 1] == ISLAND {
                visit_cells(&mut cloned_grid, row, grid[row].len() - 1); //eliminate E0502
            }
        }

        let mut closed_islands = 0;
        for row in 1..cloned_grid.len() - 1 {
            for col in 1..cloned_grid[row].len() - 1 {
                if cloned_grid[row][col] == ISLAND {
                    closed_islands += 1;
                    visit_cells(&mut cloned_grid, row, col);
                }
            }
        }
        closed_islands
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_1254_num_of_closed_islands::a1254::closed_island;

    #[test]
    fn t_000() {
        let inp: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1, 1, 1, 0],
                                      vec![1, 0, 0, 0, 0, 1, 1, 0],
                                      vec![1, 0, 1, 0, 1, 1, 1, 0],
                                      vec![1, 0, 0, 0, 0, 1, 0, 1],
                                      vec![1, 1, 1, 1, 1, 1, 1, 0]];
        let ans: i32 = 2;

        assert_eq!(ans, closed_island(inp));
    }

    #[test]
    fn t_001() {
        let inp: Vec<Vec<i32>> = vec![vec![0, 0, 1, 0, 0],
                                      vec![0, 1, 0, 1, 0],
                                      vec![0, 1, 1, 1, 0]];

        let ans: i32 = 1;

        assert_eq!(ans, closed_island(inp));
    }

    #[test]
    fn t_002() {
        let inp: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1, 1, 1],
                                      vec![1, 0, 0, 0, 0, 0, 1],
                                      vec![1, 0, 1, 1, 1, 0, 1],
                                      vec![1, 0, 1, 0, 1, 0, 1],
                                      vec![1, 0, 1, 1, 1, 0, 1],
                                      vec![1, 0, 0, 0, 0, 0, 1],
                                      vec![1, 1, 1, 1, 1, 1, 1]];
        let ans: i32 = 2;

        assert_eq!(ans, closed_island(inp));
    }
}