#[allow(dead_code)]
pub fn title_to_number(column_title: String) -> i32 {
    let mut number = 0;
    let reverse: Vec<char> = column_title.chars().rev().collect();

    for i in 0..reverse.len() {
        let cur_char_val = reverse[i] as i32 - 64;

        number += cur_char_val * 26i32.pow(i as u32);
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("A");
        let expected = 1;
        let result = title_to_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("AB");
        let expected = 28;
        let result = title_to_number(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = String::from("ZY");
        let expected = 701;
        let result = title_to_number(entry);
        assert_eq!(result, expected);
    }
}