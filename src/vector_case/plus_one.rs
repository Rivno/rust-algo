#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; digits.len() + 1];
    let mut add = 1;
    for x in (0..digits.len()).rev() {
        let val = digits[x];
        if val + add < 10 {
            result[x + 1] = val + add;
            add = 0;
        }
    }

    if add == 0 {
        result.remove(0);
    } else {
        result[0] = 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![1,2,3];
        let expected = vec![1,2,4];
        let result = plus_one(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![4,3,2,1];
        let expected = vec![4,3,2,2];
        let result = plus_one(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![9];
        let expected = vec![1,0];
        let result = plus_one(entry);
        assert_eq!(result, expected);
    }
}