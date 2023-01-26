use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();

    for i in 0..nums.len() {
        if let Some(n) = hash.get(&nums[i]) {
            return vec![*n as i32, i as i32];
        }

        hash.insert(target - nums[i], i);
    }
    
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![2,7,11,15];
        let target = 9;
        let expected = vec![0, 1];
        let result = two_sum(entry, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![3,2,4];
        let target = 6;
        let expected = vec![1, 2];
        let result = two_sum(entry, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        let result = two_sum(entry, target);
        assert_eq!(result, expected);
    }
}