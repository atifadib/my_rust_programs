struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn new() -> Solution{
        Solution {}
    }

    pub fn min_depth(&self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = self.min_depth(node.borrow().left.clone());
                let right = self.min_depth(node.borrow().right.clone());

                if left == 0 || right == 0{
                    return left.max(right) + 1;
                }
                return left.min(right) + 1;
            },
            None => {return 0;},
        }
    }
}

fn main(){
    let _solution = Solution::new();
    //print!("{:?}", solution);
}