#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut index = 0usize;

    for i in 1..nums.len() {
        if nums[index] != nums[i] {
            index += 1;
            nums[index] = nums[i];
        }
    }

    return (index + 1) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut entry = vec![1, 1, 2];
        let target = 2;
        let expected = vec![1, 2];
        let result = remove_duplicates(&mut entry);
        assert_eq!(result, target);
        assert_eq!(&entry[0..result as usize], expected);
    }

    #[test]
    fn case2() {
        let mut entry = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let target = 5;
        let expected = vec![0, 1, 2, 3, 4];
        let result = remove_duplicates(&mut entry);
        assert_eq!(result, target);
        assert_eq!(&entry[0..result as usize], expected);
    }
}