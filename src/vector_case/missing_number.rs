#[allow(dead_code)]
pub fn missing_number(nums: Vec<i32>) -> i32 {
    /*
        The bitwise XOR (^) operator returns a 1 in each bit position for which the corresponding bits of either but not both operands are 1s.

        a = 5;  // 101
        b = 3;  // 011
        a ^ b   // 110
        Expected output: 6
    */

    let mut value= 0;
    for i in 0..nums.len() {
        value ^= (i as i32 + 1) ^ nums[i];
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![3,0,1];
        let expected = 2;
        let result = missing_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![0,1];
        let expected = 2;
        let result = missing_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![9,6,4,2,3,5,7,0,1];
        let expected = 8;
        let result = missing_number(entry);
        assert_eq!(result, expected);
    }
}