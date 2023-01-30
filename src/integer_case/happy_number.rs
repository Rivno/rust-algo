use std::collections::HashSet;

#[allow(dead_code)]
pub fn is_happy(n: i32) -> bool {
    let mut result = n;
    let mut set = HashSet::new();

    while result != 1 {
        let mut tmp_result = 0;
        loop {
            tmp_result += (result % 10) * (result % 10);
            result = result / 10;

            if result < 1 {
                break;
            }
        }

        if set.contains(&tmp_result) {
            return false;
        }

        set.insert(tmp_result);

        result = tmp_result;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 19;
        let expected = true;
        let result = is_happy(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 2;
        let expected = false;
        let result = is_happy(entry);
        assert_eq!(result, expected);
    }
}