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
fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(n)) => Some(n),
        (Some(n), None) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next)
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2))
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(4))),
                        })),
                    })),
                })),
            })),
        }));

        let result = merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let list1 = None;
        let list2 = None;
        let expected = None;
        let result = merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));
        let expected = Some(Box::new(ListNode::new(0)));
        let result = merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
}