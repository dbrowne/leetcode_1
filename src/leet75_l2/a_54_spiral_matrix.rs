/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an m x n matrix, return all elements of the matrix in spiral order.
//
//
//
// Example 1:
//
//
// Input: matrix = [[1,2,3],
//                  [4,5,6],
//                  [7,8,9]]
// Output: [1,2,3,6,9,8,7,4,5]
// Example 2:
//
//
// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//
//
// Constraints:
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 10
// -100 <= matrix[i][j] <= 100


mod p2 {

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let h = matrix.len();
        let w = matrix[0].len();
        let mut ret_vec = vec![];
        if  h < 1{ return ret_vec}
        let  mut x_min = 0;
        let  mut x_max = h;
        let  mut y_min = 0;
        let  mut y_max = w;


        loop {
            for y in y_min..y_max  {ret_vec.push(matrix[x_min][y]);}
            x_min +=1;
            if x_min==x_max {break; }

            for  x in x_min..x_max  {ret_vec.push(matrix[x][y_max-1]);}
            y_max -=1;
            if  y_min == y_max {break;}

            for  y in (y_min..y_max).rev()  {ret_vec.push(matrix[x_max-1][y]);}
            x_max -=1;
            if x_min == x_max { break;}
            for  x in (x_min..x_max).rev()  {ret_vec.push(matrix[x][y_min]) }

            y_min+=1;
            if y_min==y_max  {break; }

        }


        ret_vec
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_l2::a_54_spiral_matrix::p2::spiral_order;

    #[test]
    fn test_00() {
        let inp:Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let ans = vec![1,2,3,4,8,12,11,10,9,5,6,7];
        assert_eq!(ans, spiral_order(inp));
    }
}