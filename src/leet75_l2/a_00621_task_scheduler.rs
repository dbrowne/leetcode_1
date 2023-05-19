/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */
// https://leetcode.com/problems/task-scheduler
//
// Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
//
// However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.
//
// Return the least number of units of times that the CPU will take to finish all the given tasks.
//
//
//
// Example 1:
//
// Input: tasks = ['A','A','A','B','B','B'], n = 2
// Output: 8
// Explanation:
// A -> B -> idle -> A -> B -> idle -> A -> B
// There is at least 2 units of time between any two same tasks.
// Example 2:
//
// Input: tasks = ['A','A','A','B','B','B'], n = 0
// Output: 6
// Explanation: On this case any permutation of size 6 would work since n = 0.
// ['A','A','A','B','B','B']
// ['A','B','A','B','A','B']
// ['B','B','B','A','A','A']
// ...
// And so on.
// Example 3:
//
// Input: tasks = ['A','A','A','A','A','A','B','C','D','E','F','G'], n = 2
// Output: 16
// Explanation:
// One possible solution is
// A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
//
//
// Constraints:
//
// 1 <= task.length <= 104
// tasks[i] is upper-case English letter.
// The integer n is in the range [0, 100].
pub mod a621 {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        const MAX_CHARS: usize = 26;
        const START_CHAR: usize = 'A' as usize;
        let tsk_cnt: i32 = tasks.len() as i32;
        let mut char_v = vec![0; MAX_CHARS];
        let mut max = -1;
        let mut count = 0;
        for tsk in tasks {
            char_v[tsk as usize - START_CHAR] += 1;
            let char_count: i32 = char_v[tsk as usize - START_CHAR];
            if char_count == max {
                count += 1;
            } else if char_count > max {
                count = 1;
                max = char_count;
            }
        }
        let ans = max * (n + 1) - n + count - 1;
        if ans < tsk_cnt {
            tsk_cnt
        } else {
            ans }
    }


    pub fn least_interval_faster(tasks: Vec<char>, n: i32) -> i32 {
        use  std::cmp::max;
        let base_ascii: u8 = 65;
        let mut counter: [i32; 26] = [0; 26];
        let mut max_count: i32 = 0;
        let mut num_max: i32 = 0;

        for task in &tasks {
            let index = *task as u8 - base_ascii;
            let ascii_index = index as usize;
            counter[ascii_index] += 1;
            if max_count < counter[ascii_index] {
                max_count = counter[ascii_index];
                num_max = 1;
            } else if max_count == counter[ascii_index] {
                num_max += 1;
            } else {
                continue;
            }
        }

        let part_count = max_count - 1;
        let part_length = n - (num_max - 1);
        let empty_slots = part_count * part_length;
        let available = tasks.len() as i32 - max_count * num_max;
        let idles = max(0, empty_slots - available);
        print!("{}", num_max);
        tasks.len() as i32 + idles
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_l2::a_00621_task_scheduler::a621::least_interval;

    #[test]
    fn t_001() {
        assert_eq!(16, least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2));
    }
}