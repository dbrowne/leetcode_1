/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */



// We are given an array asteroids of integers representing asteroids in a row.
//
// For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.
//
// Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
//
//
//
// Example 1:
//
// Input: asteroids = [5,10,-5]
// Output: [5,10]
// Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
// Example 2:
//
// Input: asteroids = [8,-8]
// Output: []
// Explanation: The 8 and -8 collide exploding each other.
// Example 3:
//
// Input: asteroids = [10,2,-5]
// Output: [10]
// Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
//
//
// Constraints:
//
// 2 <= asteroids.length <= 104
// -1000 <= asteroids[i] <= 1000
// asteroids[i] != 0


mod  a735{
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        for &asteroid in asteroids.iter() {
            answer.push(asteroid);
            while answer.len() > 1 && answer[answer.len() - 2] > 0 && answer[answer.len() - 1] < 0 {
                let l = answer.pop().unwrap();
                let r = answer.pop().unwrap();
                match r.cmp(&-l) {
                    std::cmp::Ordering::Less => answer.push(l),
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => answer.push(r),
                }
            }
        }
        answer
    }
}


#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00735_asteroid_collision::a735::asteroid_collision;

    #[test]
    fn t_00(){
        assert_eq!(vec![5,10],asteroid_collision(vec![5,10,-5]) );
    }
}