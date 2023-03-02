use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut result = 0;
    let mut index_head = 0usize;
    let mut index_current = 0usize;
    let chars: Vec<char> = s.chars().collect();
    let mut map = HashMap::new();

    while index_current < s.len() {
        let char = chars[index_current];
        if let Some(char_index) = map.get(&char) {
            let res = index_current - index_head;
            println!("{:?} - {:?} - {:?}", res, index_current, index_head);
            if result < res {
                result = res;
            }

            if char_index + 1 > index_head {
                index_head = char_index + 1;
            }

            map.entry(char).and_modify(|i| *i = index_current);
        } else {
            map.insert(char, index_current);
        }

        index_current += 1;
    }

    let res = index_current - index_head;
    if result < res {
        result = res;
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("abcabcbb");
        let expected = 3;
        let result = length_of_longest_substring(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("bbbbb");
        let expected = 1;
        let result = length_of_longest_substring(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from("pwwkew");
        let expected = 3;
        let result = length_of_longest_substring(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case4() {
        let entry = String::from("abba");
        let expected = 2;
        let result = length_of_longest_substring(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case5() {
        let entry = String::from("aab");
        let expected = 2;
        let result = length_of_longest_substring(entry);
        assert_eq!(result, expected);
    }
}
