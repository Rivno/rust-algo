#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min = prices[0];

    for price in prices {
        if price < min {
            min = price;
        }
        if profit < price - min {
            profit = price - min;
        }
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entry = vec![7,1,5,3,6,4];
        let expected = 5;
        let result = max_profit(entry);
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let entry = vec![7,6,4,3,1];
        let expected = 0;
        let result = max_profit(entry);
        assert_eq!(result, expected);
    }
}