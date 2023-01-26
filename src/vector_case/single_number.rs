#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    /*
        The bitwise XOR (^) operator returns a 1 in each bit position for which the corresponding bits of either but not both operands are 1s.

        a = 5;  // 101
        b = 3;  // 011
        a ^ b   // 110
        Expected output: 6
    */
    nums.into_iter().fold(0, |acc, num| acc ^ num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![2,2,1];
        let expected = 1;
        let result = single_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![4,1,2,1,2];
        let expected = 4;
        let result = single_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![1];
        let expected = 1;
        let result = single_number(entry);
        assert_eq!(result, expected);
    }
}