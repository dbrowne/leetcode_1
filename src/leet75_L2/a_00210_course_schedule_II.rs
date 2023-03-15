/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/course-schedule-ii/?
// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
//
// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
//
//
//
// Example 1:
//
// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: [0,1]
// Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
// Example 2:
//
// Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
// Output: [0,2,1,3]
// Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
// Example 3:
//
// Input: numCourses = 1, prerequisites = []
// Output: [0]
//
//
// Constraints:
//
// 1 <= numCourses <= 2000
// 0 <= prerequisites.length <= numCourses * (numCourses - 1)
// prerequisites[i].length == 2
// 0 <= ai, bi < numCourses
// ai != bi
// All the pairs [ai, bi] are distinct.

pub mod a210 {

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let tst = prerequisites.is_empty() || prerequisites[0].is_empty();

        if tst && num_courses == 1 {
            return vec![0];
        }
        if tst && num_courses == 2 {
            return vec![1, 0];
        }
        let mut graph: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            graph[p[0] as usize].0 += 1;
            graph[p[1] as usize].1.push(p[0] as usize);
        }
        let mut stack: Vec<usize> = Vec::new();
        for (i, e) in (0..).zip(graph.iter()) {
            if e.0 == 0 {
                stack.push(i);
            }
        }
        let mut ret_val: Vec<i32> = Vec::with_capacity(num_courses as usize);
        while let Some(last) = stack.pop() {
            ret_val.push(last as i32);
            for i in graph[last].1.clone() {
                graph[i].0 -= 1;
                if graph[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        if ret_val.len() == num_courses as usize {
            ret_val
        } else {
            vec![]
        }
    }

    #[derive(Copy, Clone)]
    enum Status {
        ToDo,
        InProcess,
        Done,
    }
    // Faster solution

    struct Solution {}

    impl Solution {
        pub fn dfs(
            graph: &Vec<Vec<usize>>,
            statuses: &mut Vec<Status>,
            cycle: &mut bool,
            cell: usize,
            res: &mut Vec<i32>,
        ) {
            println!("{}", cell);
            statuses[cell] = Status::InProcess;
            for out in graph[cell].iter() {
                match statuses[*out] {
                    Status::ToDo => Solution::dfs(graph, statuses, cycle, *out, res),
                    Status::InProcess => {
                        *cycle = true;
                    }
                    _ => {}
                }
            }
            statuses[cell] = Status::Done;
            res.push(cell as i32);
        }

        pub fn find_order(num_courses: i32, prs: Vec<Vec<i32>>) -> Vec<i32> {
            let num_courses = num_courses as usize;
            let mut graph: Vec<Vec<usize>> = vec![vec![]; num_courses];
            for it in prs.iter() {
                graph[it[0] as usize].push(it[1] as usize);
            }

            let mut statuses: Vec<Status> = vec![Status::ToDo; num_courses];
            let mut cycle = false;

            let mut res: Vec<i32> = Vec::new();
            for i in 0..num_courses {
                match statuses[i] {
                    Status::Done => {}
                    _ => Solution::dfs(&graph, &mut statuses, &mut cycle, i, &mut res),
                }
            }
            if cycle {
                return Vec::new();
            };
            res
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_L2::a_00210_course_schedule_II::a210::find_order;

    #[test]
    fn t_0001() {
        let inp = vec![vec![1, 0]];
        let ans = vec![0, 1];
        assert_eq!(ans, find_order(2, inp))
    }

    #[test]
    fn t_0002() {
        let inp = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let ans = vec![0, 2, 1, 3];
        assert_eq!(ans, find_order(4, inp))
    }

    #[test]
    fn t_0003() {
        let inp = vec![vec![]];
        let ans = vec![0];

        assert_eq!(ans, find_order(1, inp));
    }
}

