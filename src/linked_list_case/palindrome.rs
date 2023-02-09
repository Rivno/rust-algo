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

    fn get_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        for val in vec.iter().rev() {
            head = Some(Box::new(ListNode { val: val.clone(), next: head }));
        }

        head
    }

    #[test]
    fn case1() {
        let entry = get_linked_list(vec![1,2,2,1]);
        let expected = true;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = get_linked_list(vec![1,2]);
        let expected = false;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }
}