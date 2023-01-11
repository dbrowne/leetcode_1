/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/bus-routes/
//
// You are given an array routes representing bus routes where routes[i] is a bus route that the ith bus repeats forever.
//
// For example, if routes[0] = [1, 5, 7], this means that the 0th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.
// You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. You can travel between bus stops by buses only.
//
// Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.
//
//
//
// Example 1:
//
// Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
// Output: 2
// Explanation: The best strategy is take the first bus to the bus stop 7, then take the second bus to the bus stop 6.
// Example 2:
//
// Input: routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
// Output: -1
//
//
// Constraints:
//
// 1 <= routes.length <= 500.
// 1 <= routes[i].length <= 105
// All the values of routes[i] are unique.
// sum(routes[i].length) <= 105
// 0 <= routes[i][j] < 106
// 0 <= source, target < 106

mod  a915{
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut bus_stop_bus_route = std::collections::HashMap::new();
        for (route_index, bus_route) in routes.iter().enumerate() {
            for bus_stop in bus_route {
                bus_stop_bus_route
                    .entry(*bus_stop)
                    .or_insert_with(|| std::collections::HashSet::new())
                    .insert(route_index);
            }
        }

        let mut visited_bus_stop = std::collections::HashSet::new();
        visited_bus_stop.insert(source);
        let mut visited_route = std::collections::HashSet::new();
        let mut num_buses = 0;

        let mut journey = std::collections::VecDeque::new();
        journey.push_back((source, num_buses));

        while let Some((bus_stop, num_buses)) = journey.pop_front() {
            if bus_stop == target {
                return num_buses;
            }
            for next_route in bus_stop_bus_route[&bus_stop].iter() {
                if !visited_route.contains(next_route) {
                    visited_route.insert(*next_route);
                    for new_bus_stop in routes[*next_route].iter() {
                        if !visited_bus_stop.contains(new_bus_stop) {
                            visited_bus_stop.insert(*new_bus_stop);
                            journey.push_back((*new_bus_stop, num_buses + 1));
                        }
                    }
                }
            }
        }
        -1
    }


    use std::collections::{VecDeque, HashSet,HashMap};
    use std::iter::repeat;

    pub fn num_buses_to_destination_faster(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0
        }

        let mut table = HashMap::new();
        for (bus,route) in routes.iter().enumerate() {
            for station in route {
                table.entry(station).or_insert(Vec::new()).push(bus);
            }
        }

        let mut visited: HashSet<_> = table[&source].iter().collect();
        let mut q = table[&source].iter().zip(repeat(1)).collect::<VecDeque<_>>();
        while let Some((&bus, steps)) = q.pop_front() {
            for &station in &routes[bus] {
                if station == target {
                    return steps;
                }
                for bus in &table[&station] {
                    if !visited.contains(bus) {
                        q.push_back((bus, steps+1));
                        visited.insert(bus);
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00915_bus_routes::a915::num_buses_to_destination;

    #[test]
    fn t_0000(){
        assert_eq!(0,num_buses_to_destination(vec![vec![0]],1,2) );
    }
}