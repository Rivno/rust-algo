#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    let mut fibonacci = vec![0; (n+2) as usize];

    fibonacci[1] = 1;
    fibonacci[2] = 2;

    for i in 3usize..(n+1) as usize {
        fibonacci[i] = fibonacci[i - 2] + fibonacci[i - 1];
    }

    fibonacci[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 2;
        let expected = 2;
        let result = climb_stairs(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 3;
        let expected = 3;
        let result = climb_stairs(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = 6;
        let expected = 13;
        let result = climb_stairs(entry);
        assert_eq!(result, expected);
    }
}