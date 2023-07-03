struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }
    pub fn apply_operations(&self, nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = [].to_vec();
        let mut index = 0;
        while index < nums.len() {
            if nums[index] == 0{
                index += 1
            }
            else if index + 1 >= nums.len(){
                answer.push(nums[index]);
                index += 1
            }
            else if nums[index] == nums[index+1]{
                answer.push(nums[index]*2);
                index += 2
            }
            else{
                answer.push(nums[index]);
                index += 1
            }
        }
        while answer.len() != nums.len(){
            answer.push(0);
        }
        return answer
    }
}

fn main(){
    let solution = Solution::new();
    print!("{:?}", solution.apply_operations(vec![1,2,2,1,0,1]))
}