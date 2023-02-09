#[allow(dead_code)]
pub fn is_power_of_three(n: i32) -> bool {
    ((n as f64).log10() / 3f64.log10()).fract() == 0f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 243;
        let expected = true;
        let result = is_power_of_three(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 0;
        let expected = false;
        let result = is_power_of_three(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = -1;
        let expected = false;
        let result = is_power_of_three(entry);
        assert_eq!(result, expected);
    }
}