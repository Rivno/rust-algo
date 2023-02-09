#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut swap_index = 0usize;
    let length = nums.len();
    for i in 0..length {
        if nums[i] != 0 {
            nums[swap_index] = nums[i];
            swap_index += 1;
        }
    }

    for i in swap_index..length {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut entry = vec![0,1,0,3,12];
        let expected = vec![1,3,12,0,0];
        move_zeroes(&mut entry);
        assert_eq!(entry, expected);
    }

    #[test]
    fn case2() {
        let mut entry = vec![0];
        let expected = vec![0];
        move_zeroes(&mut entry);
        assert_eq!(entry, expected);
    }
}