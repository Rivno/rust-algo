#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    /*
        The left shift (<<) operator shifts the first operand the specified number of bits, modulo 32, to the left.
        Excess bits shifted off to the left are discarded. Zero bits are shifted in from the right.

        a = 5   // 00101
        b = 2   // 00010
        a << b  // 10100
        Expected output: 20
    */
    
    let mut number = 0;
    let len = nums.len();

    for bit_index in 0..32 {
        let mut count = 0;

        for num in nums.iter() {
            if num & (1 << bit_index) != 0 {
                count += 1;
            }
        }

        if count > len / 2 {
            number += 1 << bit_index;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![3,2,3];
        let expected = 3;
        let result = majority_element(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![2,2,1,1,1,2,2];
        let expected = 2;
        let result = majority_element(entry);
        assert_eq!(result, expected);
    }
}