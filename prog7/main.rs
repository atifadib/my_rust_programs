struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }
    pub fn search_insert(&self, nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        // let mut ans = -1;

        while start <= end {
            let mid = ((start + end) / 2) as usize;
            if mid >= nums.len(){
                break;
            }
            if nums[mid] == target {
                return mid as i32;
            }
            else if nums[mid] > target{
                end = mid - 1;
            }
            else {
                start = mid + 1;
            }
        }
        return start as i32
        
    }
}

fn main(){
    let solution = Solution::new();
    print!("{:?}", solution.search_insert(vec![1,2,35,60], 25));
}