pub mod two {
    use std::cmp::min;

    pub fn find_min_falling_path_sum(
        matrix: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if col < 0 || col == matrix.len() {
            return i32::MAX;
        }
        if row == matrix.len() - 1 {
            return matrix[row][col]; // reached the last row
        }

        if memo[row][col] != i32::MAX {
            return memo[row][col];
        }

        let left = find_min_falling_path_sum(matrix, row + 1, col, memo);
        let middle = find_min_falling_path_sum(matrix, row + 1, col + 1, memo);
        let mut right = i32::MAX;
        if col > 0 {
            right = find_min_falling_path_sum(matrix, row + 1, col - 1, memo);
        }
        memo[row][col] = min(left, min(middle, right)) + matrix[row][col];
        memo[row][col]
    }

    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut min_falling_sum = i32::MAX;

        let mut memo: Vec<Vec<i32>> = vec![vec![i32::MAX; matrix[0].len()]; matrix.len()];

        for startCol in 0..matrix.len() {
            min_falling_sum = min(
                min_falling_sum,
                find_min_falling_path_sum(&matrix, 0, startCol, &mut memo),
            );
        }

        min_falling_sum
    }

    pub fn min_falling_path_sum_fast(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());

        for i in (0..n - 1).rev() {
            matrix[i][0] += min(matrix[i + 1][0], matrix[i + 1][1]);

            for j in i..m - 1 {
                if j > 0 {
                    matrix[i][j] += min(
                        matrix[i + 1][j],
                        min(matrix[i + 1][j - 1], matrix[i + 1][j + 1]),
                    );
                }
            }
            matrix[i][m - 1] += min(matrix[i + 1][m - 1], matrix[i + 1][m - 2]);
        }
        matrix
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0931_minimum_falling_path_sum::two::{
        min_falling_path_sum, min_falling_path_sum_fast,
    };

    #[test]
    fn test_01() {
        let inp: Vec<Vec<i32>> = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(min_falling_path_sum(inp), 13);
    }

    #[test]
    fn test_02() {
        let inp: Vec<Vec<i32>> = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(min_falling_path_sum_fast(inp), 13);
    }
}
