#[allow(dead_code)]
fn get_value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    }
}

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut last_char_val = 0;
    let mut int_val = 0;
    let chars: Vec<char> = s.chars().collect();

    for x in (0..s.len()).rev() {
        let val = get_value(chars[x]);
        if val < last_char_val {
            int_val -= val;
        } else {
            int_val += val;
        }
        last_char_val = val;
    }

    int_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("III");
        let expected = 3;
        let result = roman_to_int(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("LVIII");
        let expected = 58;
        let result = roman_to_int(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from("MCMXCIV");
        let expected = 1994;
        let result = roman_to_int(entry);
        assert_eq!(result, expected);
    }
}