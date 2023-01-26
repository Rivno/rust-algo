#[allow(dead_code)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut vec_prefix = vec![];

    let mut strs = strs;
    let rest_words = strs.split_off(1);
    let first_word = strs.pop().unwrap_or(String::from(""));
    for i in 0..first_word.len() {
        vec_prefix.push(&first_word[0..=i]);
    }

    while let Some(prefix) = vec_prefix.pop() {
        let mut has_break = false;
        for word in rest_words.iter() {
            if prefix.len() > word.len() || &word[0..prefix.len()] != prefix {
                has_break = true;
                break;
            }
        }

        if !has_break {
            return prefix.to_string();
        }
    }

    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        let expected = String::from("fl");
        let result = longest_common_prefix(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![String::from("dog"), String::from("racecar"), String::from("car")];
        let expected = String::from("");
        let result = longest_common_prefix(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        let entry = vec![String::from("a")];
        let expected = String::from("a");
        let result = longest_common_prefix(entry);
        assert_eq!(result, expected);
    }
}