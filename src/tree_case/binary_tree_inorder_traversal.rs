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
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(n) => {
            let node = n.borrow();
            let left = inorder_traversal(node.left.clone());
            let right = inorder_traversal(node.right.clone());
            let mut traversal = vec![];
            traversal.extend(left);
            traversal.push(node.val);
            traversal.extend(right);
            traversal
        },
        None => vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let expected = vec![1, 3, 2];
        let result = inorder_traversal(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = None;
        let expected = vec![];
        let result = inorder_traversal(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let expected = vec![1];
        let result = inorder_traversal(entry);
        assert_eq!(result, expected);
    }
}