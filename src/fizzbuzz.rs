pub struct Solution;
// https://leetcode-cn.com/problems/fizz-buzz/solution/bi-jiao-jian-ji-de-xie-fa-by-ignorant-k-yqur/

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|x| {
                let mut s = String::new();
                if x % 3 == 0 {
                    s += "Fizz";
                }
                if x % 5 == 0 {
                    s += "Buzz";
                }
                if s.is_empty() {
                    s = x.to_string()
                }

                s
            })
            .collect()
    }
}

mod tests {
    #[test]
    fn fizz_buzz_3() {
        let n = 3;
        let result = super::Solution::fizz_buzz(n);
        assert_eq!(result, vec!["1", "2", "Fizz"]);
    }
    #[test]
    fn fizz_buzz_5() {
        let n = 5;
        let result = super::Solution::fizz_buzz(n);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }
    #[test]
    fn fizz_buzz_15() {
        let n = 15;
        let result = super::Solution::fizz_buzz(n);
        assert_eq!(
            result,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
