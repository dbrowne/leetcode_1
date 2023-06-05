/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
//
// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
// Note:
//
// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
// Only the filled cells need to be validated according to the mentioned rules.
//
//
// Example 1:
//
//
// Input: board =
// [["5","3",".",".","7",".",".",".","."]
// vec!["6",".",".","1","9","5",".",".","."]
// vec![".","9","8",".",".",".",".","6","."]
// vec!["8",".",".",".","6",".",".",".","3"]
// vec!["4",".",".","8",".","3",".",".","1"]
// vec!["7",".",".",".","2",".",".",".","6"]
// vec![".","6",".",".",".",".","2","8","."]
// vec![".",".",".","4","1","9",".",".","5"]
// vec![".",".",".",".","8",".",".","7","9"]]
// Output: true
// Example 2:
//
// Input: board =
// [["8","3",".",".","7",".",".",".","."]
// vec!["6",".",".","1","9","5",".",".","."]
// vec![".","9","8",".",".",".",".","6","."]
// vec!["8",".",".",".","6",".",".",".","3"]
// vec!["4",".",".","8",".","3",".",".","1"]
// vec!["7",".",".",".","2",".",".",".","6"]
// vec![".","6",".",".",".",".","2","8","."]
// vec![".",".",".","4","1","9",".",".","5"]
// vec![".",".",".",".","8",".",".","7","9"]]
// Output: false
// Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//
//
// Constraints:
//
// board.length == 9
// board[i].length == 9
// board[i][j] is a digit 1-9 or '.'.


pub mod a36 {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut cols: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut boxes: Vec<Vec<bool>> = vec![vec![false; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let num: usize = board[i][j].to_digit(10).unwrap() as usize - 1;
                    let k: usize = (i / 3) * 3 + j / 3;
                    if rows[i][num] || cols[j][num] || boxes[k][num] {
                        return false;
                    }
                    rows[i][num] = true;
                    cols[j][num] = true;
                    boxes[k][num] = true;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn t_00() {
        let board: Vec<Vec<char>> = vec![vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                                         vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                                         vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                                         vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                                         vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                                         vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                                         vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                                         vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                                         vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'], ];
        assert_eq!(super::a36::is_valid_sudoku(board), true);
    }

    #[test]
    fn  t_01(){
        let  board:Vec<Vec<char>> = vec![vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                                         vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                                         vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                                         vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                                         vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                                         vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                                         vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                                         vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                                         vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'], ];
        assert_eq!(super::a36::is_valid_sudoku(board), false);
    }
}

