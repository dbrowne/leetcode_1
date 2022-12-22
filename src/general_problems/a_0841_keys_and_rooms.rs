/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. Your goal is to visit all the rooms.
// However, you cannot enter a locked room without having its key.
//
// When you visit a room, you may find a set of distinct keys in it.
// Each key has a number on it, denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.
//
// Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited room i, return true if you can visit all the rooms, or false otherwise.
//
//
//
// Example 1:
//
// Input: rooms = [[1],[2],[3],[]]
// Output: true
// Explanation:
// We visit room 0 and pick up key 1.
// We then visit room 1 and pick up key 2.
// We then visit room 2 and pick up key 3.
// We then visit room 3.
// Since we were able to visit every room, we return true.
// Example 2:
//
// Input: rooms = [[1,3],[3,0,1],[2],[0]]
// Output: false
// Explanation: We can not enter room number 2 since the only key that unlocks it is in that room.
//
//
// Constraints:
//
// n == rooms.length
// 2 <= n <= 1000
// 0 <= rooms[i].length <= 1000
// 1 <= sum(rooms[i].length) <= 3000
// 0 <= rooms[i][j] < n
// All the values of rooms[i] are unique.


pub  mod  xxx{
    pub  fn  can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let  mut ky_stack: Vec<i32> = Vec::new();
        let  mut visited:Vec<bool>=vec![false;rooms.len()];

        ky_stack.push(0);
        visited[0] = true;


        while !ky_stack.is_empty(){
            let  room_key = ky_stack.pop().unwrap();
            visited[room_key as usize] = true;
            for new_key in rooms[room_key  as usize].iter(){

                if  !visited[*new_key as usize]{
                    visited[*new_key as usize] = true;
                    for i in rooms[*new_key as usize].iter() {
                        ky_stack.push(*i);
                    }
                }
            }

        }

        for entered in visited.iter()  {
            if  !entered{
                return false;
            }
        }


        true
    }

    pub fn can_visit_all_rooms_fast(rooms: Vec<Vec<i32>>) -> bool {
        let mut remaining = rooms.len();
        let mut unlocked = vec![false; rooms.len()];
        let mut keys = vec![];

        remaining -= 1;
        unlocked[0] = true;
        keys.push(0);

        while let Some(room) = keys.pop() {
            for &next_room in rooms[room].iter() {
                let next_room = next_room as usize;
                if !unlocked[next_room] {
                    remaining -= 1;
                    if remaining == 0 {
                        return true;
                    }
                    unlocked[next_room] = true;
                    keys.push(next_room);
                }
            }
        }
        false
    }


}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_0841_keys_and_rooms::xxx::can_visit_all_rooms;

    #[test]
    fn test_01(){
        assert_eq!(true, can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]))
    }
    #[test]
    fn test_02(){
        assert_eq!(true, can_visit_all_rooms(vec![vec![2],vec![],vec![1]]))
    }


}