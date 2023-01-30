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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = None;
    let mut current = head;

    while let Some(current_node) = current {
        let next_node = current_node.next;
        new_head = Some(Box::new(ListNode { val: current_node.val , next: new_head }));
        current = next_node;
    }

    new_head
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
            next: add_node(ListNode {
                val: 2,
                next:add_node(ListNode {
                    val: 3,
                    next:add_node(ListNode {
                        val: 4,
                        next: add_node(ListNode::new(5)),
                    }),
                }),
            }),
        });
        let expected = add_node(ListNode {
            val: 5,
            next: add_node(ListNode {
                val: 4,
                next:add_node(ListNode {
                    val: 3,
                    next:add_node(ListNode {
                        val: 2,
                        next: add_node(ListNode::new(1)),
                    }),
                }),
            }),
        });
        let result = reverse_list(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = add_node(ListNode {
            val: 1,
            next: add_node(ListNode::new(2)),
        });
        let expected = add_node(ListNode {
            val: 2,
            next: add_node(ListNode::new(1)),
        });
        let result = reverse_list(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = None;
        let expected = None;
        let result = reverse_list(entry);
        assert_eq!(result, expected);
    }
}