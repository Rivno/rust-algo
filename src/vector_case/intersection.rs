use std::collections::HashMap;

#[allow(dead_code)]
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut inter = vec![];
    let mut hash = HashMap::new();

    for num in nums1 {
        hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    for num in nums2 {
        match hash.get(&num) {
            Some(0) => continue,
            Some(_count) => {
                inter.push(num);
                hash.entry(num).and_modify(|count| *count -= 1);
            }
            None => continue,
        }
    }

    inter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry1 = vec![1,2,2,1];
        let entry2 = vec![2,2];
        let expected = vec![2, 2];
        let result = intersect(entry1, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry1 = vec![4,9,5];
        let entry2 = vec![9,4,9,8,4];
        let expected = vec![9, 4];
        let result = intersect(entry1, entry2);
        assert_eq!(result, expected);
    }
}