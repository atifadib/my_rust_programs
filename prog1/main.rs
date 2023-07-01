use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }

    pub fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        let mut index = 0;
        for num in nums.iter() {
            if let Some(&prev_index) = seen.get(num) {
                return vec![prev_index, index];
            }
            seen.insert(target - num, index);
            index += 1;
        }
        Vec::new()
    }
}

fn main() {
    let soln = Solution::new();
    println!("{:?}", soln.two_sum(vec![1, 2, 3, 4], 5));
}

