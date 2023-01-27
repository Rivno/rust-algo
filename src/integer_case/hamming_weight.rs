#[allow(dead_code)]
pub fn hamming_weight (n: u32) -> i32 {
    let mut number = 0;

    for bit_index in 0..32 {
        if n & (1 << bit_index) != 0 {
            number += 1;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 19; // 00000000000000000000000000001011
        let expected = 3;
        let result = hamming_weight(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 256; // 00000000000000000000000010000000
        let expected = 1;
        let result = hamming_weight(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = 4294967293; // 11111111111111111111111111111101
        let expected = 31;
        let result = hamming_weight(entry);
        assert_eq!(result, expected);
    }
}