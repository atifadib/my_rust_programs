use std::collections::HashSet;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }

    pub fn longest_common_prefix(&self, strs: Vec<String>) -> String {
        if strs.len() == 1{
            return String::from(&strs[0]);
        }
        let mut set: HashSet<char> = HashSet::new();
        let mut index = 0;

        // find min iteration length
        let mut min_size = 200;
        for item in strs.iter(){
            min_size = cmp::min(min_size, item.len());
        }

        while index < min_size {
            for item in strs.iter(){
                let curr_char = item.chars().nth(index);
                match curr_char {
                    Some(ch) => {set.insert(ch);},
                    None => {print!("Invalid Index");}
                }
            }
            if set.len() > 1{
                break
            }   
            index += 1;
            set.clear();
        }
        let p = &strs[0];
        // print!("{:?}", &p[0..index]);
        if index == min_size{
            return String::from(&p[0..min_size]);
        }
        if index >= p.len(){
            return String::from("");
        }
        return String::from(&p[0..index]);
    }
}

fn main(){
    let soln = Solution::new();
    let out = soln.longest_common_prefix(vec![String::from("atif"),String::from("ftia"),String::from("atiz")]);
    print!("{:?}", out);
}