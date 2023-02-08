use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut hash = HashMap::new();

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    for i in 0..s.len() {
        hash.entry(s_bytes[i]).and_modify(|count| *count += 1).or_insert(1);
        hash.entry(t_bytes[i]).and_modify(|count| *count -= 1).or_insert(-1);
    }

    for (_, count) in hash {
        if count != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = String::from("anagram");
        let entry2 = String::from("nagaram");
        let expected = true;
        let result = is_anagram(entry, entry2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = String::from("rat");
        let entry2 = String::from("car");
        let expected = false;
        let result = is_anagram(entry, entry2);
        assert_eq!(result, expected);
    }
}
