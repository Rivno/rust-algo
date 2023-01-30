use std::collections::HashSet;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![1,2,3,1];
        let expected = true;
        let result = contains_duplicate(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![1,2,3,4];
        let expected = false;
        let result = contains_duplicate(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![1,1,1,3,3,4,3,2,4,2];
        let expected = true;
        let result = contains_duplicate(entry);
        assert_eq!(result, expected);
    }
}