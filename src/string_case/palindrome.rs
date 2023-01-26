#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    let text = s.to_lowercase()
        .replace(|x: char| !x.is_alphanumeric(), "");
    let reverse: String = text.chars().rev().collect();

    text == reverse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("A man, a plan, a canal: Panama");
        let expected = true;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("race a car");
        let expected = false;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from(" ");
        let expected = true;
        let result = is_palindrome(entry);
        assert_eq!(result, expected);
    }
}
