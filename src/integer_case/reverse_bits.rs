#[allow(dead_code)]
pub fn reverse_bits(x: u32) -> u32 {
    let mut number = 0;

    let mut index_end = 31;
    for bit_index in 0..32 {
        if x & (1 << bit_index) != 0 {
            number += 1 << index_end;
        }

        if bit_index < 31 {
            index_end -= 1;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 43261596; // 00000010100101000001111010011100
        let expected = 964176192; // 00111001011110000010100101000000
        let result = reverse_bits(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 4294967293; // 11111111111111111111111111111101
        let expected = 3221225471; // 10111111111111111111111111111111
        let result = reverse_bits(entry);
        assert_eq!(result, expected);
    }
}