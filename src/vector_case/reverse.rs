#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    let mid = length / 2;
    let mut index_end = length - 1;
    for i in 0..mid {
        s.swap(i, index_end);
        index_end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut entry = vec!['h','e','l','l','o'];
        let expected = vec!['o','l','l','e','h'];
        reverse_string(&mut entry);
        assert_eq!(entry, expected);
    }

    #[test]
    fn case2() {
        let mut entry = vec!['H','a','n','n','a','h'];
        let expected = vec!['h','a','n','n','a','H'];
        reverse_string(&mut entry);
        assert_eq!(entry, expected);
    }
}