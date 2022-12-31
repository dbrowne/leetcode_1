/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub  mod a_14{

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans:String = String::from("");
        let mut pos:usize = 0;
        let mut hit: bool = false;

        if strs.is_empty() || strs[0].is_empty() {
            return ans;
        }

        loop{
            ans.push_str(&strs[0][pos..pos +1]);
            for i in 0..strs.len()  {
                if strs[i].len() < pos +1 || strs[i][0..pos +1] !=ans {
                    hit = true;
                    break;
                }
            }
            match hit {
                true => break ans[0..pos].to_string(),
                false => if  pos +1 == strs[0].len() {
                    break ans;
                }
            }
            pos +=1;

        }
    }

}

#[cfg(test)]
mod test{
    use crate::leet75_L2::a_00014_longest_common_prefix::a_14::longest_common_prefix;
    #[test]
    fn test_000(){
        let inp:Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
        let ans:String = String::from("fl");
        assert_eq!(ans,longest_common_prefix(inp) );
    }
}