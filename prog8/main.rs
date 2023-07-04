struct Solution {}


impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }
    pub fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        let size = nums.len();
        while index < size{
            if index < nums.len() && nums[index] == val{
                nums.remove(index);
            }
            else {
                index += 1;
            }
        }
        return nums.len() as i32;
    }
}

fn main(){
    let solution = Solution::new();
    let mut my_vec: Vec<i32> = vec![1,2,3,4,3,3];
    print!("{:?}", solution.remove_element(&mut my_vec, 3));
    print!("\n New Vector: {:?}", my_vec);
}