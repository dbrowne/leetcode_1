/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */
//
// https://leetcode.com/problems/number-of-provinces/
//
//
// There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
//
// A province is a group of directly or indirectly connected cities and no other cities outside of the group.
//
// You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.
//
// Return the total number of provinces.
//
//
//
// Example 1:
//
//
// Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
// Output: 2
// Example 2:
//
//
// Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
// Output: 3
//
//
// Constraints:
//
// 1 <= n <= 200
// n == isConnected.length
// n == isConnected[i].length
// isConnected[i][j] is 1 or 0.
// isConnected[i][i] == 1
// isConnected[i][j] == isConnected[j][i]

pub  mod  a547{
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {


        0
    }

    // fn  dfs(&mut M:Vec<Vec<i32>, &mut visited:Vec<i32>, idx:usize){
    //     for jdx  in 0..M[0].len()  {
    //         if M[idx][jdx] ==1 &&visited[jdx] ==0 {
    //
    //         }
    //
    //     }
   //   }
}

#[cfg(test)]
mod  test {
    use crate::leet75_l2::a_00547_number_of_provinces::a547::find_circle_num;

    #[test]
    fn t_000(){
        let inp = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        assert_eq!(2, find_circle_num(inp));
    }

    #[test]
    fn t_001(){
        let inp = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(3, find_circle_num(inp));
    }
}