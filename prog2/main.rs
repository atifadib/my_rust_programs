struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
    pub fn is_palindrome(&self, mut x: i32) -> bool {
        let mut prev = 0;
        let tmp = x;
        while x > 0{
            prev = (prev * 10) + x % 10;
            x = (x / 10) as i32;
        }
        if prev == tmp{
            return true
        }
        return false
    }
}


fn main() {
    let soln = Solution::new();
    println!("{:?}", soln.is_palindrome(131));
    println!("{:?}", soln.is_palindrome(120));
}

