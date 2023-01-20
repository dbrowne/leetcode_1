/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
//
// Implement the Trie class:
//
// Trie() Initializes the trie object.
// void insert(String word) Inserts the string word into the trie.
// boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
// boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
//
//
// Example 1:
//
// Input
// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
// Output
// [null, null, true, false, true, null, true]
//
// Explanation
// Trie trie = new Trie();
// trie.insert("apple");
// trie.search("apple");   // return True
// trie.search("app");     // return False
// trie.startsWith("app"); // return True
// trie.insert("app");
// trie.search("app");     // return True
//
//
// Constraints:
//
// 1 <= word.length, prefix.length <= 2000
// word and prefix consist only of lowercase English letters.
// At most 3 * 104 calls in total will be made to insert, search, and startsWith.

pub  mod  a208{
    use std::collections::HashMap;
    /*
        TrieNode Implemention
    */
    struct TrieNode {
        children: HashMap<char,TrieNode>,
        is_word: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            TrieNode {children: HashMap::new(), is_word: false}
        }
    }


    /*
        Trie Implementation
    */

    struct Trie {
        root: TrieNode,
    }


    /**
      * `&self` means the method takes an immutable reference
      * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

        fn new() -> Self {
            Trie {root: TrieNode::new()}
        }

        fn insert(&mut self, word: String) {
            let mut current_node = &mut self.root;

            for c in word.chars() {
                let next_node = current_node.children.entry(c)
                                            .or_insert(TrieNode::new());
                current_node = next_node;
            }
            current_node.is_word = true;
        }

        fn search(&self, word: String) -> bool {
            let mut current_node = &self.root;

            for c in word.chars() {
                match current_node.children.get(&c) {
                    Some(next_node) => current_node = next_node,
                    None => return false,
                }
            }

            return current_node.is_word;
        }

        fn starts_with(&self, prefix: String) -> bool {
            let mut current_node = &self.root;

            for c in prefix.chars() {
                match current_node.children.get(&c) {
                    Some(next_node) => current_node = next_node,
                    None => return false,
                }
            }

            return true;
        }
    }

    // /**
    //  * Your Trie object will be instantiated and called as such:
    //  * let obj = Trie::new();
    //  * obj.insert(word);
    //  * let ret_2: bool = obj.search(word);
    //  * let ret_3: bool = obj.starts_with(prefix);
    //  */



    // #[derive(Default)]
    // pub struct Trie2 {
    //     children: [Option<Box<Trie>>; 26],
    //     is_end: bool,
    // }
    //
    // impl Trie2 {
    //     pub fn new() -> Self {
    //         Default::default()
    //     }
    //
    //     pub fn insert(&mut self, word: String) {
    //         let mut curr = self;
    //
    //         for c in word.chars() {
    //             let i = (c as u8 - b'a') as usize;
    //
    //             if curr.children[i].is_none() {
    //                 curr.children[i] = Some(Box::new(Trie::new()));
    //             }
    //
    //             curr = curr.children[i].as_mut().unwrap();
    //         }
    //
    //         curr.is_end = true;
    //     }
    //
    //     pub fn search2(&self, word: String) -> bool {
    //         let mut curr = self;
    //
    //         for c in word.chars() {
    //             let i = (c as u8 - b'a') as usize;
    //
    //             if let Some(node) = curr.children[i].as_ref() {
    //                 curr = node;
    //             } else {
    //                 return false;
    //             }
    //         }
    //
    //         curr.is_end
    //     }
    //
    //     pub fn starts_with2(&self, prefix: String) -> bool {
    //         let mut curr = self;
    //
    //         for c in prefix.chars() {
    //             let i = (c as u8 - b'a') as usize;
    //
    //             if let Some(node) = curr.children[i].as_ref() {
    //                 curr = node;
    //             } else {
    //                 return false;
    //             }
    //         }
    //
    //         true
    //     }
    // }
}