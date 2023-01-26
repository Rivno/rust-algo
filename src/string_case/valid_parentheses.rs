#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for char in s.chars() {
        match char {
            '(' => stack.push(char),
            '[' => stack.push(char),
            '{' => stack.push(char),
            ')' => {
                match stack.pop() {
                    Some('(') => continue,
                    _ => return false,
                }
            },
            ']' => {
                match stack.pop() {
                    Some('[') => continue,
                    _ => return false,
                }
            },
            '}' => {
                match stack.pop() {
                    Some('{') => continue,
                    _ => return false,
                }
            },
            _ => continue,
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("()");
        let expected = true;
        let result = is_valid(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("()[]{}");
        let expected = true;
        let result = is_valid(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from("(]");
        let expected = false;
        let result = is_valid(entry);
        assert_eq!(result, expected);
    }
}
