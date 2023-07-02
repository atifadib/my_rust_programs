struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }

    pub fn remove_duplicates(&self, nums: &mut Vec<i32>) -> i32 {
        let mut index = 1;
        while index < nums.len(){
            if nums[index] == nums[index-1]{
                nums.remove(index);
            }
            else{
                index += 1
            }
        }
        return nums.len() as i32;
    }
}

fn main(){
    let solution = Solution::new();
    let mut my_vec: Vec<i32> = [1,1,2,3,4,4].to_vec();
    print!("{:?}", solution.remove_duplicates(&mut my_vec));
}