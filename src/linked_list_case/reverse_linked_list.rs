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

    fn get_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        for val in vec.iter().rev() {
            head = Some(Box::new(ListNode { val: val.clone(), next: head }));
        }

        head
    }

    #[test]
    fn case1() {
        let entry = get_linked_list(vec![1,2,3,4,5]);
        let expected = get_linked_list(vec![5,4,3,2,1]);
        let result = reverse_list(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = get_linked_list(vec![1,2]);
        let expected = get_linked_list(vec![2,1]);
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