use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }
    pub fn climb_stairs(&self, n: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        
        fn climb(n: i32, hm: &mut HashMap<i32, i32>) -> i32{
            let value = hm.get(&n);
            match value {
                Some(&value) => {return value;},
                None => {}
            }
            if n < 0 {
                return 0;
            }
            else if n == 0{
                return 1;
            }
            else {
                let ret = climb(n-1, hm) + climb(n-2, hm);
                hm.insert(n, ret);
                return ret;
            }
        }
        return climb(n, &mut hm);

    }
}

fn main(){
    let solution = Solution::new();
    print!("{:?}", solution.climb_stairs(12));
}