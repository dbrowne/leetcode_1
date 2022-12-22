/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


pub  mod a_10{
    pub fn is_match(s: String, p: String) -> bool {
        let  slen = s.len();
        let  plen = p.len();

        let mut vals = vec![vec![false; plen + 1]; slen + 1];
        vals[0][0] = true;

        for i in 0..slen {

        }

        true
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn test_01(){
        assert_eq!(0, 0);
    }
}