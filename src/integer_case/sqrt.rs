#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    let mut low = 0;
    let mut high = x;

    while low <= high {
        let mid = ((low + high) as f64 / 2f64).floor() as i32;
        if (mid as i64 * mid as i64) > x as i64 {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    high
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 4;
        let expected = 2;
        let result = my_sqrt(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 8;
        let expected = 2;
        let result = my_sqrt(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = 2147395599;
        let expected = 46339;
        let result = my_sqrt(entry);
        assert_eq!(result, expected);
    }
}