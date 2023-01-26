#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
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



#[allow(dead_code)]
pub fn add_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}

#[allow(dead_code)]
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }

    if nums.len() == 1 {
        return add_node(TreeNode { val: nums[0], left: None, right: None });
    }

    if nums.len() == 2 {
        return add_node(TreeNode {
            val: nums[1], 
            left: add_node(TreeNode { val: nums[0], left: None, right: None }),
            right: None,
        });
    }

    if nums.len() == 3 {
        return add_node(TreeNode {
            val: nums[1], 
            left: add_node(TreeNode { val: nums[0], left: None, right: None }),
            right: add_node(TreeNode { val: nums[2], left: None, right: None }),
        });
    }

    let mid;
    if nums.len() % 2 == 0 {
        mid = (nums.len() / 2) - 1;
    } else {
        mid = (nums.len() as f64 / 2f64).floor() as usize;
    }

    let left_vec = &nums[0..mid];
    let right_vec = &nums[(mid+1)..nums.len()];

    println!("here => {mid:?} {:?}", nums[mid]);
    add_node(TreeNode {
        val: nums[mid], 
        left: sorted_array_to_bst(left_vec.to_vec()),
        right: sorted_array_to_bst(right_vec.to_vec()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![-10, -3, 0, 5, 9];
        let expected = add_node(TreeNode {
            val: 0,
            left: add_node(TreeNode {
                val: -3,
                left: add_node(TreeNode {
                    val: -10,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
            right: add_node(TreeNode {
                val: 9,
                left: add_node(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
        });
        let result = sorted_array_to_bst(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![1, 3];
        let expected = add_node(TreeNode {
            val: 3,
            left: add_node(TreeNode {
                val: 1,
                left: None,
                right: None,
            }),
            right: None,
        });
        let result = sorted_array_to_bst(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![-1,0,1,2];
        let expected = add_node(TreeNode {
            val: 0,
            left: add_node(TreeNode {
                val: -1,
                left: None,
                right: None,
            }),
            right: add_node(TreeNode {
                val: 2,
                left: add_node(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
        });
        let result = sorted_array_to_bst(entry);
        assert_eq!(result, expected);
    }
}