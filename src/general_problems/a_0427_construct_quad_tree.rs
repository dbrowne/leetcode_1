/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://en.wikipedia.org/wiki/Quadtree

// Given a n * n matrix grid of 0's and 1's only. We want to represent the grid with a Quad-Tree.
//
// Return the root of the Quad-Tree representing the grid.
//
// Notice that you can assign the value of a node to True or False when isLeaf is False, and both are accepted in the answer.
//
// A Quad-Tree is a tree data structure in which each internal node has exactly four children. Besides, each node has two attributes:
//
// val: True if the node represents a grid of 1's or False if the node represents a grid of 0's.
// isLeaf: True if the node is leaf node on the tree or False if the node has the four children.
// class Node {
// public boolean val;
// public boolean isLeaf;
// public Node topLeft;
// public Node topRight;
// public Node bottomLeft;
// public Node bottomRight;
// }
// We can construct a Quad-Tree from a two-dimensional area using the following steps:
//
// If the current grid has the same value (i.e all 1's or all 0's) set isLeaf True and set val to the value of the grid and set the four children to Null and stop.
// If the current grid has different values, set isLeaf to False and set val to any value and divide the current grid into four sub-grids as shown in the photo.
// Recurse for each of the children with the proper sub-grid.
//
// If you want to know more about the Quad-Tree, you can refer to the wiki.
//
// Quad-Tree format:
//
// The output represents the serialized format of a Quad-Tree using level order traversal, where null signifies a path terminator where no node exists below.
//
// It is very similar to the serialization of the binary tree. The only difference is that the node is represented as a list [isLeaf, val].
//
// If the value of isLeaf or val is True we represent it as 1 in the list [isLeaf, val] and if the value of isLeaf or val is False we represent it as 0.
//
//
//
// Example 1:
//
//
// Input: grid = [[0,1],[1,0]]
// Output: [[0,1],[1,0],[1,1],[1,1],[1,0]]
// Explanation: The explanation of this example is shown below:
// Notice that 0 represnts False and 1 represents True in the photo representing the Quad-Tree.
//
// Example 2:
//
//
//
// Input: grid = [[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,1,1,1,1],[1,1,1,1,1,1,1,1],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0]]
// Output: [[0,1],[1,1],[0,1],[1,1],[1,0],null,null,null,null,[1,0],[1,0],[1,1],[1,1]]
// Explanation: All values in the grid are not the same. We divide the grid into four sub-grids.
// The topLeft, bottomLeft and bottomRight each has the same value.
// The topRight have different values so we divide it into 4 sub-grids where each has the same value.
// Explanation is shown in the photo below:
//
//
//
// Constraints:
//
// n == grid.length == grid[i].length
// n == 2x where 0 <= x <= 6

// pub  mod  a427{
// 
//     struct Quadtree {
//         root: Option<Box<QuadNode>>,
//     }
// 
//     struct QuadNode {
//         bounds: Bounds,
//         nodes: Option<[Box<QuadNode>; 4]>,
//         elements: Vec<Element>,
//     }
// 
//     struct Bounds {
//         x: f32,
//         y: f32,
//         width: f32,
//         height: f32,
//     }
// 
//     struct Element {
//         x: f32,
//         y: f32,
//     }
// 
//     impl Quadtree {
//         fn new(bounds: Bounds) -> Quadtree {
//             Quadtree { root: Some(Box::new(QuadNode::new(bounds))) }
//         }
//     }
// 
//     impl QuadNode {
//         fn new(bounds: Bounds) -> QuadNode {
//             QuadNode { bounds: bounds, nodes: None, elements: Vec::new() }
//         }
// 
//         fn insert(&mut self, element: Element) {
//             // If the element is outside of the bounds of this node, ignore it
//             if !self.bounds.contains(element) {
//                 return;
//             }
// 
//             // If this node does not have any child nodes, add the element to its list of elements
//             if self.nodes.is_none() {
//                 self.elements.push(element);
// 
//                 // If the number of elements in this node exceeds some threshold, split it into child nodes
//                 if self.elements.len() > MAX_ELEMENTS_PER_NODE {
//                     self.subdivide();
//                 }
//             }
//             // Otherwise, insert the element into one of its child nodes
//             else {
//                 let index = self.get_index(element);
//                 self.nodes.as_mut().unwrap()[index].insert(element);
//             }
//         }
// 
//         fn subdivide(&mut self) {
//             let bounds = self.bounds;
//             let x = bounds.x;
//             let y = bounds.y;
//             let width = bounds.width / 2.0;
//             let height = bounds.height / 2.0;
// 
//             let nw = Box::new(QuadNode::new(Bounds { x: x, y: y, width: width, height: height }));
//             let ne = Box::new(QuadNode::new(Bounds { x: x + width, y: y, width: width, height: height }));
//             let sw = Box::new(QuadNode::new(Bounds { x: x, y: y + height, width: width, height: height }));
//             let se = Box::new(QuadNode::new(Bounds { x: x + width, y: y + height, width: width, height: height }));
// 
//             self.nodes = Some([nw, ne, sw, se]);
// 
//             // Move elements to child nodes
//             for element in self.elements.drain(..) {
//                 let index = self.get_index(element);
//                 self.nodes.as_mut().unwrap()[index].insert(element);
//             }
//         }
// 
//         // fn get_index(&self, element: Element) -> usize {
//         //     let bounds = self.bounds;
//         //     let x = element.x;
//         //     let y = element.y;
//         //     let vertical_midpoint = bounds.x + (bounds.width / 2.0);
//         //     let horizontal_midpoint = bounds.y + (bounds.height / 2.0);
//         //
//         //     if x < vertical_midpoint {
//         //         if y < horizontal_midpoint {
//         //             0 // NW
//         //         } else {
//         //             2 // SW
//         //         }
//         //     } else {
//         //         if y < horizontal_midpoint {
//         //             1 // NE
//         //         } else {
//         //             3 // SE
//         //
//         //
//         //             pub  fn  construct(grid :Vec<Vec<i32>>) ->Node{
// 
//     }
// }