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
pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = Box::new(ListNode::new(0));
    let mut tail = res.as_mut();
    let mut rest = 0;
    while l1 != None || l2 != None || rest != 0 {
        let mut val = rest;
        if let Some(node) = l1 {
            l1 = node.next;
            val += node.val;
        }
        if let Some(node) = l2 {
            l2 = node.next;
            val += node.val;
        }

        rest = val / 10;
        val = val % 10;

        let node = Box::new(ListNode::new(val));
        tail.next = Some(node);
        tail = tail.next.as_mut()?;
    }
    res.next
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
        let entry1 = get_linked_list(vec![2,4,3]);
        let entry2 = get_linked_list(vec![5,6,4]);
        let expected = get_linked_list(vec![7,0,8]);
        let result = add_two_numbers(entry1, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry1 = get_linked_list(vec![0]);
        let entry2 = get_linked_list(vec![0]);
        let expected = get_linked_list(vec![0]);
        let result = add_two_numbers(entry1, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry1 = get_linked_list(vec![9,9,9,9,9,9,9]);
        let entry2 = get_linked_list(vec![9,9,9,9]);
        let expected = get_linked_list(vec![8,9,9,9,0,0,0,1]);
        let result = add_two_numbers(entry1, entry2);
        assert_eq!(result, expected);
    }
}