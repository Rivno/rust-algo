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
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(n) => {
            let node = n.borrow();
            let left_depth = max_depth(node.left.clone());
            let right_depth = max_depth(node.right.clone());

            if left_depth > right_depth {
                return left_depth + 1;
            }
            return right_depth + 1;
        },
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn case1() {
        let entry = add_node(TreeNode {
            val: 3,
            left: add_node(TreeNode {
                val: 9,
                left: None,
                right: None,
            }),
            right: add_node(TreeNode {
                val: 20,
                left: add_node(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }),
                right: add_node(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }),
            }),
        });
        let expected = 3;
        let result = max_depth(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = add_node(TreeNode {
            val: 1,
            left: None,
            right: add_node(TreeNode {
                val: 2,
                left: None,
                right: None,
            }),
        });
        let expected = 2;
        let result = max_depth(entry);
        assert_eq!(result, expected);
    }
}