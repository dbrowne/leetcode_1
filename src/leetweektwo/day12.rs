pub mod day_12 {
    use std::collections::HashMap;

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let s_len = s.len();
        let p_len = p.len();

        if s_len < p_len {
            return ans;
        }
        let get_char_index = |c: char| (c as u8 - 'a' as u8) as usize;
        let mut pattern = [0_u8; 26];
        for c in p.chars() {
            pattern[get_char_index(c)] += 1;
        }
        let mut window = [0_u8; 26];
        for c in s[..p_len].chars() {
            window[get_char_index(c)] += 1;
        }

        let s_vec: Vec<char> = s.chars().collect();
        for i in 0..s_len - p_len {
            if pattern == window {
                ans.push(i as i32);
            }
            window[get_char_index(s_vec[i])] -= 1;
            window[get_char_index(s_vec[i + p_len])] += 1;
        }
        if pattern == window {
            ans.push((s_len - p_len) as i32);
        }

        ans
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_vec: Vec<char> = s.chars().collect();
        let mut freq_hash: HashMap<char, usize> = HashMap::new();
        let mut lo = 0;
        let mut ans = 0;
        for r in 0..s_vec.len() {
            *freq_hash.entry(s_vec[r]).or_default() += 1;
            let max_freq = freq_hash.values().max().unwrap();
            let max_rep = (r - lo + 1) - max_freq;
            if max_rep > k as usize {
                *freq_hash.entry(s_vec[lo]).or_default() -= 1;
                lo += 1;
            }
            ans = usize::max(ans, r - lo + 1);
        }

        ans as i32
    }

    pub fn character_replacement_fast(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut freq_cnt = vec![0; 26];
        let mut max_freq = 0;
        let mut ans = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            let s_c = s_chars[right];
            let s_c_idx = s_c as u32 - 'A' as u32;
            freq_cnt[s_c_idx as usize] += 1;
            max_freq = max_freq.max(freq_cnt[s_c_idx as usize]);
            let is_valid = (right + 1 - left - max_freq) <= k as usize;
            if !is_valid {
                let outgoing_char = s_chars[left];
                let outgoing_char_idx = outgoing_char as u32 - 'A' as u32;
                freq_cnt[outgoing_char_idx as usize] -= 1;
                left += 1;
            }
            ans = right + 1 - left;
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::day12::day_12::find_anagrams;

    #[test]
    fn test_0() {
        let ans = find_anagrams(String::from("abab"), String::from("ab"));
        assert_eq!(3, ans.len());
    }
}
