// https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x2cv1c/

pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        if digits.is_empty() {
            return digits;
        }

        let mut result = vec![];
        let mut i = digits.len() - 1;

        let n1 = digits[i] + 1;
        let mut carry = n1 / 10;
        let remainder1 = n1 % 10;
        result.insert(0, remainder1);

        loop {
            if i == 0 {
                break;
            } else {
                i -= 1;
            }

            let n = digits[i] + carry;
            let remainder = n % 10;
            carry = n / 10;
            result.insert(0, remainder);
        }

        if carry > 0 {
            result.insert(0, carry);
        }
        result
    }
}

mod tests {
    #[test]
    fn plus_ex1() {
        let digits = vec![1, 2, 3];
        let result = super::Solution::plus_one(digits);
        let expected = vec![1, 2, 4];

        assert_eq!(result, expected);
    }
    #[test]
    fn plus_ex2() {
        let digits = vec![4, 3, 2, 1];
        let result = super::Solution::plus_one(digits);
        assert_eq!(result, vec![4, 3, 2, 2]);
    }
    #[test]
    fn plus_ex3() {
        let digits = vec![4, 3, 2, 9];
        let result = super::Solution::plus_one(digits);
        assert_eq!(result, vec![4, 3, 3, 0]);
    }
    #[test]
    fn plus_ex4() {
        let digits = vec![9, 9, 9, 9];
        let result = super::Solution::plus_one(digits);
        assert_eq!(result, vec![1, 0, 0, 0, 0]);
    }
}
