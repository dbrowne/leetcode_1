pub mod ns1 {
    use std::cmp;

    pub fn longest_palindrome(s: String) -> String {
        const MAX_LEN: usize = 2004;
        let seq: Vec<char> = s.chars().collect();
        let mut len = seq.len();
        if len <= 1 {
            return s;
        }

        let mut s_new: [char; MAX_LEN] = ['\0'; MAX_LEN];
        let mut p: [usize; MAX_LEN] = [0; MAX_LEN];
        s_new[0] = '$';
        s_new[1] = '#';
        let mut j = 2;

        for i in 0..len {
            s_new[j] = seq[i];
            j += 1;

            s_new[j] = '#';
            j += 1;
        }

        s_new[j] = '\0';
        len = j;

        let mut index: usize = 0;
        let mut max_len: usize = 0;
        let mut center: usize = 0;
        let mut max_right: usize = 0;

        for i in 1..len {
            if i < max_right {
                p[i] = cmp::min(p[2 * center - i], max_right - i);
            } else {
                p[i] = 1;
            }

            while s_new[i - p[i]] == s_new[i + p[i]] {
                p[i] += 1;
            }
            if max_right < (p[i] + i) {
                center = i;
                max_right = p[i] + i;
            }

            if max_len < p[i] {
                max_len = p[i] - 1;
                index = (2 * i - max_right) / 2;
            }
        }
        let end: usize = index + max_len;
        s[index..end].to_owned()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_0() {
            assert_eq!(String::from("bb"), longest_palindrome(String::from("cbb")));
        }
    }
}
