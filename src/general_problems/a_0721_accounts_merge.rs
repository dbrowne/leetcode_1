/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given a list of accounts where each element accounts[i] is a list of strings, where
// the first element accounts[i][0] is a name, and the rest of the elements are emails
// representing emails of the account.
//
// Now, we would like to merge these accounts. Two accounts definitely belong to the same person
// if there is some common email to both accounts. Note that even if two accounts have the same name,
// they may belong to different people as people could have the same name. A person can have any
// number of accounts initially, but all of their accounts definitely have the same name.
//
// After merging the accounts, return the accounts in the following format: the first element of each
// account is the name, and the rest of the elements are emails in sorted order.
// The accounts themselves can be returned in any order.
//
//
//
// Example 1:
//
// Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],
// ["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],
// ["John","johnnybravo@mail.com"]]
//
// Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],
// ["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
// Explanation:
// The first and second John's are the same person as they have the common email "johnsmith@mail.com".
// The third John and Mary are different people as none of their email addresses are used by other accounts.
// We could return these lists in any order, for example the answer
// [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'],
// ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
// Example 2:
//
// Input: accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],
// ["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],
// ["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],
// ["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
// Output: [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],
// ["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],
// ["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
//
//
// Constraints:
//
// 1 <= accounts.length <= 1000
// 2 <= accounts[i].length <= 10
// 1 <= accounts[i][j].length <= 30
// accounts[i][0] consists of English letters.
// accounts[i][j] (for j > 0) is a valid email.


pub mod a721 {
    #[derive(Debug, Clone)]
    //todo:  Complete this code!!
    struct DisSet {
        item: Vec<usize>,
        rank: Vec<usize>
    }

    impl DisSet {
        fn new(n: usize) -> Self {
                        let mut item: Vec<usize> = vec![0; n];
            let mut rank: Vec<usize> = vec![0; n];
            for ctr in 0..n {
                item[ctr] = ctr;
                rank[ctr] = 0;
            }
            Self { item, rank }
        }
        fn find(&mut self, n: usize) -> usize {
            if self.item[n] == n {
                return n;
            }
            self.item[n] = self.find(self.item[n]);
            self.item[n]
        }

        fn merge(&mut self, n: usize, m: usize) {
            let n_root = self.find(n);
            let m_root = self.find(m);
        }
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        vec![vec!["".to_string()]]
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn t_000() {
        let inp: Vec<Vec<String>> = vec![
            //todo:  FIX THIS CODE
            vec!["John".to_string(), "johnsmith@mail.com".to_string(),
                 "john_newyork@mail.com".to_string()],
            vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john00@mail.com".to_string()],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()]];
        let ans: Vec<Vec<String>> = vec![vec!["Mary".to_string(), "mary@mail.com".to_string()],
                                         vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                                        vec! ["John".to_string(), "john00@mail.com".to_string(),
                                             "john_newyork@mail.com".to_string(), "johnsmith@mail.com".to_string()]];


//
// vec!["John".to_string(),"johnnybravo@mail.com".to_string()]];
    }
}