#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);
        
    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m-1];
            m -= 1;
        }
        else {
            nums1[m + n -1] = nums2[n - 1];
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut entry1 = vec![1,2,3,0,0,0];
        let num1 = 3;
        let mut entry2 = vec![2,5,6];
        let num2 = 3;
        let expected = vec![1,2,2,3,5,6];
        merge(&mut entry1, num1, &mut entry2, num2);
        assert_eq!(entry1, expected);
    }

    #[test]
    fn case2() {
        let mut entry1 = vec![1];
        let num1 = 1;
        let mut entry2 = vec![];
        let num2 = 0;
        let expected = vec![1];
        merge(&mut entry1, num1, &mut entry2, num2);
        assert_eq!(entry1, expected);
    }

    #[test]
    fn case3() {
        let mut entry1 = vec![0];
        let num1 = 0;
        let mut entry2 = vec![1];
        let num2 = 1;
        let expected = vec![1];
        merge(&mut entry1, num1, &mut entry2, num2);
        assert_eq!(entry1, expected);
    }

    #[test]
    fn case4() {
        let mut entry1 = vec![2, 0];
        let num1 = 1;
        let mut entry2 = vec![1];
        let num2 = 1;
        let expected = vec![1, 2];
        merge(&mut entry1, num1, &mut entry2, num2);
        assert_eq!(entry1, expected);
    }

    #[test]
    fn case5() {
        let mut entry1 = vec![-1,0,0,3,3,3,0,0,0];
        let num1 = 6;
        let mut entry2 = vec![1,2,2];
        let num2 = 3;
        let expected = vec![-1,0,0,1,2,2,3,3,3];
        merge(&mut entry1, num1, &mut entry2, num2);
        assert_eq!(entry1, expected);
    }
}