#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut stack = vec![];
    let mut current = head;

    while let Some(current_node) = current {
        stack.push(current_node.val);
        current = current_node.next;
    }

    let length = stack.len();
    for i in 0..length / 2 {
        if stack[i] != stack[length - 1 -i] {
            return false;
        }
    }

    true
}



#[cfg(test)]
mod tests {
    use super::*;

    fn add_node(node: ListNode) -> Option<Box<ListNode>> {
        Some(Box::new(node))
    }

    #[test]
    fn case1() {
        let entry = add_node(ListNode {
            val: 1,
            next:add_node(ListNode {
                val: 2,
                next:add_node(ListNode {
                    val: 2,
                    next: add_node(ListNode::new(1)),
                }),
            }),
        });
        let expected = true;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = add_node(ListNode {
            val: 1,
            next: add_node(ListNode::new(2)),
        });
        let expected = false;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }
}