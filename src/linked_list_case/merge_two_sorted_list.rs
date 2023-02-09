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

    fn get_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        for val in vec.iter().rev() {
            head = Some(Box::new(ListNode { val: val.clone(), next: head }));
        }

        head
    }

    #[test]
    fn case1() {
        let entry1 = get_linked_list(vec![1,2,4]);
        let entry2 = get_linked_list(vec![1,3,4]);
        let expected = get_linked_list(vec![1,1,2,3,4,4]);
        let result = merge_two_lists(entry1, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry1 = None;
        let entry2 = None;
        let expected = None;
        let result = merge_two_lists(entry1, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry1 = None;
        let entry2 = get_linked_list(vec![0]);
        let expected = get_linked_list(vec![0]);
        let result = merge_two_lists(entry1, entry2);
        assert_eq!(result, expected);
    }
}