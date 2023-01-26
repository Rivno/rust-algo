
#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    
    result.push(vec![1]);
    if num_rows == 1 {
        return result;
    }

    for level in 2..=num_rows as usize {
        let mut row = vec![];
        let prev_row = &result[level - 2];
        
        for index_row in 0..level {
            let val;
            if index_row == 0 || index_row == level - 1 {
                val = 1;
            } else { 
                val = prev_row[index_row -1] + prev_row[index_row];
            }
            
            row.push(val);
        }

        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = 5;
        let expected = vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1], vec![1,4,6,4,1]];
        let result = generate(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = 1;
        let expected = vec![vec![1]];
        let result = generate(entry);
        assert_eq!(result, expected);
    }
}