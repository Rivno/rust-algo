#[allow(dead_code)]
macro_rules! vec_string {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[allow(dead_code)]
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec![String::from(""); n as usize];

    for i in 1..=n as usize {
        match (i % 3, i % 5 ) {
            (0, 0) => result[i - 1] = String::from("FizzBuzz"),
            (0, _) => result[i - 1] = String::from("Fizz"),
            (_, 0) => result[i - 1] = String::from("Buzz"),
            _ => result[i - 1] = i.to_string(),
        } 
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 3;
        let expected = vec_string!["1","2","Fizz"];
        let result = fizz_buzz(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 5;
        let expected = vec_string!["1","2","Fizz","4","Buzz"];
        let result = fizz_buzz(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = 15;
        let expected = vec_string!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"];
        let result = fizz_buzz(entry);
        assert_eq!(result, expected);
    }
}