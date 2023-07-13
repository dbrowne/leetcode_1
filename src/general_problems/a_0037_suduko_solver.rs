/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Write a program to solve a Sudoku puzzle by filling the empty cells.
//
// A sudoku solution must satisfy all of the following rules:
//
// Each of the digits 1-9 must occur exactly once in each row.
// Each of the digits 1-9 must occur exactly once in each column.
// Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
// The '.' character indicates empty cells.
//
//
//
// Example 1:
//
//
// Input: board = [['5','3','.','.','7','.','.','.','.'],['6','.','.','1','9','5','.','.','.'],['.','9','8','.','.','.','.','6','.'],['8','.','.','.','6','.','.','.','3'],['4','.','.','8','.','3','.','.','1'],['7','.','.','.','2','.','.','.','6'],['.','6','.','.','.','.','2','8','.'],['.','.','.','4','1','9','.','.','5'],['.','.','.','.','8','.','.','7','9']]
// Output: [['5','3','4','6','7','8','9','1','2'],['6','7','2','1','9','5','3','4','8'],['1','9','8','3','4','2','5','6','7'],['8','5','9','7','6','1','4','2','3'],['4','2','6','8','5','3','7','9','1'],['7','1','3','9','2','4','8','5','6'],['9','6','1','5','3','7','2','8','4'],['2','8','7','4','1','9','6','3','5'],['3','4','5','2','8','6','1','7','9']]
// Explanation: The input board is shown above and the only valid solution is shown below:
//
//
//
//
// Constraints:
//
// board.length == 9
// board[i].length == 9
// board[i][j] is a digit or '.'.
// It is guaranteed that the input board has only one solution.
pub mod a37 {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        _ = rec_solver(board, 0, 0);
    }

    ///Recursive solver for the sudoku board
    /// Returns true if the board is solved
    /// iterates through the board column by row
    /// if the cell is not empty, skip it
    /// if the cell is empty check if the digit is valid for the cell
    /// if not valid, set the cell back to empty and return false
    fn rec_solver(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        if row == 9 {
            return true;
        }
        if col == 9 {
            return rec_solver(board, row + 1, 0);
        }

        if board[row][col] != '.' {
            return rec_solver(board, row, col + 1);
        }

        for digit in 1..=9 {
            let inp_char = char::from_digit(digit as u32, 10).unwrap();
            if is_valid(board, row, col, inp_char) {
                board[row][col] = inp_char;
                match rec_solver(board, row, col + 1) {
                    true => return true,
                    // put it back if we fail
                    false => board[row][col] = '.',
                }
            }
        }

        false
    }

    pub fn is_valid(board: &mut Vec<Vec<char>>, row: usize, col: usize, t_char: char) -> bool {
        // check the columns
        for cl in 0..9 {
            if board[row][cl] == t_char {
                return false;
            }
        }

        // check the rows
        for rw in 0..9 {
            if board[rw][col] == t_char {
                return false;
            }
        }

        //check the box
        let row_offset: usize = (row / 3) * 3;
        let col_offset: usize = (col / 3) * 3;

        for rw in row_offset..row_offset + 3 {
            for cl in col_offset..col_offset + 3 {
                if board[rw][cl] == t_char {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0037_suduko_solver::a37::solve_sudoku;

    #[test]
    fn t_01() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let ans: Vec<Vec<char>> = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        solve_sudoku(&mut board);
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board[row][col], ans[row][col]);
            }
        }
    }
}
