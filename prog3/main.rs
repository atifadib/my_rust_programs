use std::collections::HashMap;

struct Solution {}


impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }
    pub fn is_valid(&self, s: String) -> bool {
        let mut stack = vec![];
        let mut pair = HashMap::new();
        pair.insert('(', ')');
        pair.insert('{', '}');
        pair.insert('[', ']');

        for ch in s.chars() {
            if stack.is_empty() {
                stack.push(ch);
            } else {
                let top = stack[stack.len() - 1];
                if let Some(&closing) = pair.get(&top) {
                    if closing == ch {
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                } else {
                    stack.push(ch);
                }
            }
        }

        stack.is_empty()
    }
}

fn main() {
    let soln = Solution::new();
    println!("{:?}", soln.is_valid(String::from("[{}]")));
    println!("{:?}", soln.is_valid(String::from("[]{}(")));
}


