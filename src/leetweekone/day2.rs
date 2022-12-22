pub mod day_two {
    use std::collections::HashMap;

    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.chars().count() != t.chars().count() {
            return false;
        }
        // map the first 256 characters  assuming we are only dealing with
        let (mut map_st, mut map_ts) = ([0; 256], [0; 256]);
        let s_it = s.as_bytes().iter();
        let t_it = t.as_bytes().iter();
        for (sb, tb) in s_it.zip(t_it).map(|(a, b)| (*a + 1, *b + 1)) {
            let tbc = &mut map_st[sb as usize];
            let sbc = &mut map_ts[tb as usize];
            match *tbc == 0 && *sbc == 0 {
                true => {
                    *tbc = tb;
                    *sbc = sb;
                }
                false if *tbc != tb || *sbc != sb => return false,
                _ => (),
            }
        }
        true
    }

    // pub fn is_ismorphic2(s: String, t: String) -> bool {
    //     struct  char_count{
    //         c:char,
    //         v:i32,
    //     };
    //
    //     let mut t_to_s: HashMap<char, char_count> = HashMap::new();
    //     let mut s_to_t: HashMap<char, char_count> = HashMap::new();
    //     let s_it = s.chars();
    //     let t_it = t.chars();
    //
    //     for (sc, tc) in s_it.zip(t_it) {
    //         if ! s_to_t.contains_key(&sc) {
    //             s_to_t.insert(sc, char_count { c: tc, v: 1 });
    //         }else{
    //             let xx = s_to_t.get_mut(&sc).unwrap();
    //             xx.v +=1;
    //         }
    //
    //         if !t_to_s.contains_key(&tc) {
    //             t_to_s.insert(tc, char_count { c: sc, v: 1 });
    //         }else {
    //             let  xx = t_to_s.get_mut(&tc).unwrap();
    //             xx.v +=1;
    //         }
    //
    //
    //         if  (s_to_t.len() != t_to_s.len()) {
    //                 return false
    //         }
    //
    //         if  t_to_s[&tc].v != s_to_t[&sc].v{
    //             return false
    //         }
    //     }
    //     true
    // }

    pub fn is_ismorphic2(s: String, t: String) -> bool {
        let mut t_to_s: HashMap<char, char> = HashMap::new();
        let mut s_to_t: HashMap<char, char> = HashMap::new();
        let s_it = s.chars();
        let t_it = t.chars();

        for (sc, tc) in s_it.zip(t_it) {
            if s_to_t.contains_key(&sc) {
                if s_to_t[&sc] != tc {
                    return false;
                }
            } else {
                s_to_t.insert(sc, tc);
            }

            if t_to_s.contains_key(&tc) {
                if t_to_s[&tc] != sc {
                    return false;
                }
            } else {
                t_to_s.insert(tc, sc);
            }
        }

        true
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_len = s.chars().count();
        let t_len = t.chars().count();
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        let mut s_idx = 0;
        let mut t_idx = 0;
        while s_idx < s_len && t_idx < t_len {
            if s_vec[s_idx] == t_vec[t_idx] {
                s_idx += 1;
            }
            t_idx += 1;
        }
        s_idx == s_len
    }
}

#[cfg(test)]
mod tests {
    use crate::leetweekone::day2::day_two::{is_ismorphic2, is_isomorphic, is_subsequence};

    #[test]
    fn test_0() {
        assert_eq!(
            false,
            is_isomorphic(String::from("Toop"), String::from("poootper"))
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            is_isomorphic(String::from("add"), String::from("egg"))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            true,
            is_ismorphic2(String::from("papel"), String::from("title"))
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            true,
            is_ismorphic2(String::from("paper"), String::from("title"))
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            false,
            is_ismorphic2(String::from("badc"), String::from("baba"))
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(
            false,
            is_ismorphic2(String::from("bbbaaaba"), String::from("aaabbbba"))
        ); //this fails
    }

    #[test]
    fn test_6() {
        assert_eq!(
            true,
            is_subsequence(String::from("abc"), String::from("ahbgdc"))
        )
    }

    #[test]
    fn test_7() {
        assert_eq!(
            false,
            is_subsequence(String::from("axc"), String::from("ahbgdc"))
        )
    }
}
