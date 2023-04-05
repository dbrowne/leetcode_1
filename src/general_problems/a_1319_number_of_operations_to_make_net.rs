/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.

You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.

Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.



Example 1:


Input: n = 4, connections = [[0,1],[0,2],[1,2]]
Output: 1
Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.
Example 2:


Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
Output: 2
Example 3:

Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
Output: -1
Explanation: There are not enough cables.


Constraints:

1 <= n <= 105
1 <= connections.length <= min(n * (n - 1) / 2, 105)
connections[i].length == 2
0 <= ai, bi < n
ai != bi
There are no repeated connections.
No two computers are connected by more than one cable.*/

pub  mod  a1319{
    struct Solution{}


        pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
            let mut graph = vec![(false, vec![]); n as usize];
            for conn in connections.into_iter() {
                let src:usize = conn[0] as usize;
                let dst:usize = conn[1] as usize;
                graph[src].1.push(dst);
                graph[dst].1.push(src);
            }

            let mut grph:i32 = 0;
            let mut spares:i32 = 0;

            for idx in 0..graph.len() {
                if !graph[idx].0 {
                    grph += 1;
                    spares += wire_alloc(&mut graph, idx);
                }
            }

            if grph - 1 > spares {
                return -1;
            }

            grph - 1
        }

    fn wire_alloc(graph: &mut Vec<(bool, Vec<usize>)>, start: usize) -> i32 {
        let mut stack:Vec<usize> = vec![start];

        let mut spares:i32 = 0;
        while let Some(node) = stack.pop() {
            if graph[node].0 {
                spares += 1;
                continue;
            }
            graph[node].0 = true;

            for &n in graph[node].1.iter() {
                if !graph[n].0 {
                    stack.push(n);
                }
            }
        }
        spares
    }

}

#[cfg(test)]
mod  test{
    use crate::general_problems::a_1319_number_of_operations_to_make_net::a1319::make_connected;

    #[test]
    fn t_001(){
        let  n = 4;
        let  connections:Vec<Vec<i32>> = vec![vec![0,1],vec![0,2],vec![1,2]];
        let ans =1 ;

        assert_eq!(ans, make_connected(n,connections));
    }


    #[test]
    fn t_002(){
        let  n = 6;
        let  connections:Vec<Vec<i32>> = vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2],vec![1,3]];
        let  ans = 2;

        assert_eq!(ans, make_connected(n,connections));
    }


    #[test]
    fn  t_003(){
        let  n = 6;
        let  connections:Vec<Vec<i32>> = vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2]];
        let  ans = -1;
        assert_eq!(ans, make_connected(n,connections));
    }
}