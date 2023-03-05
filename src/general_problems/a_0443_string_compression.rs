// Given an array of characters chars, compress it using the following algorithm:
//
// Begin with an empty string s. For each group of consecutive repeating characters in chars:
//
// If the group's length is 1, append the character to s.
// Otherwise, append the character followed by the group's length.
// The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.
//
// After you are done modifying the input array, return the new length of the array.
//
// You must write an algorithm that uses only constant extra space.
//
//
//
// Example 1:
//
// Input: chars = ["a","a","b","b","c","c","c"]
// Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
// Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
// Example 2:
//
// Input: chars = ["a"]
// Output: Return 1, and the first character of the input array should be: ["a"]
// Explanation: The only group is "a", which remains uncompressed since it's a single character.
// Example 3:
//
// Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
// Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
// Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".
//
//
// Constraints:
//
// 1 <= chars.length <= 2000
// chars[i] is a lowercase English letter, uppercase English letter, digit, or symbol.


pub  mod  a443{
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let c_len = chars.len();
        if c_len ==1 {
            return 1;
        }

        let mut ans:usize = 0;
        let mut idx:usize = 0;
        let mut j:usize = 0;


        while idx < c_len {
            while idx < c_len && chars[idx] == chars[j] { idx += 1; }

            chars[ans] = chars[idx - 1];
            ans += 1;

            if idx > j + 1 {
                for c in (idx - j).to_string().chars() {
                    chars[ans] = c;
                    ans += 1;
                }
            }
            j = idx;
        }
        while chars.len() > ans {
            chars.pop();
        }
        ans as i32
    }
}

#[cfg(test)]
mod  test{
    use crate::general_problems::a_0443_string_compression::a443::compress;

    #[test]
    fn  t_001(){
        let  mut inp:Vec<char> = vec!['a','a','b','b','c','c','c'];
        let  ans1 =6;
        let  ans2 :Vec<char> = vec!['a','2','b','2','c','3'];
        assert_eq!(ans1,compress(&mut inp) );
        assert_eq!(ans2,inp );

    }
    
    #[test]
    fn  t_002(){
        let  mut inp:Vec<char> = vec!['a'];
        let  ans1 = 1;
        let  ans2 :Vec<char> = vec!['a'];
        assert_eq!(ans1,compress(&mut inp) );
        assert_eq!(ans2, inp);
    }

    #[test]
    fn  t_003(){
        let  mut inp:Vec<char> =vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        let  ans1 = 4;
        let  ans2:Vec<char> =vec!['a','b','1','2'];
        assert_eq!(ans1, compress(&mut inp));
        assert_eq!(ans2,inp );
    }
}