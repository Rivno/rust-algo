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
pub fn is_sym(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(right), Some(left)) => {
            let node_left = left.borrow();
            let node_right = right.borrow();
            if node_left.val != node_right.val {
                return false;
            }

            let r_left = is_sym(node_left.left.clone(), node_right.right.clone());
            let r_right = is_sym(node_left.right.clone(), node_right.left.clone());
            r_left && r_right
        },
        _ => false
    }
}

#[allow(dead_code)]
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(n) => {
            let node = n.borrow();
            is_sym(node.left.clone(), node.right.clone())
        },
        None => true
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
            val: 1,
            left: add_node(TreeNode {
                val: 2,
                left: add_node(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }),
                right: add_node(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }),
            }),
            right: add_node(TreeNode {
                val: 2,
                left: add_node(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }),
                right: add_node(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }),
            }),
        });
        let expected = true;
        let result = is_symmetric(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = add_node(TreeNode {
            val: 1,
            left: add_node(TreeNode {
                val: 2,
                left: None,
                right: add_node(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }),
            }),
            right: add_node(TreeNode {
                val: 2,
                left: None,
                right: add_node(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }),
            }),
        });
        let expected = false;
        let result = is_symmetric(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = add_node(TreeNode {
            val: 1,
            left: add_node(TreeNode {
                val: 2,
                left: add_node(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
            right: add_node(TreeNode {
                val: 2,
                left: add_node(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
        });
        let expected = false;
        let result = is_symmetric(entry);
        assert_eq!(result, expected);
    }
}