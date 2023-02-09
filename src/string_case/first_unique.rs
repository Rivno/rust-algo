#[allow(dead_code)]
pub fn first_uniq_char(s: String) -> i32 {
    if s.len() == 0 {
        return -1;
    }

    let mut alpha_count = vec![0; 26];

    for char in s.chars() {
        let index = (char as usize) - ('a' as usize);

        alpha_count[index] += 1;
    }

    for (position, char) in s.chars().enumerate() {
        let index = (char as usize) - ('a' as usize);
        
        if alpha_count[index] == 1 {
            return position as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("leetcode");
        let expected = 0;
        let result = first_uniq_char(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("loveleetcode");
        let expected = 2;
        let result = first_uniq_char(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from("aabb");
        let expected = -1;
        let result = first_uniq_char(entry);
        assert_eq!(result, expected);
    }
}