pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }
}

mod tests {
    #[test]
    fn single_num_ex1() {
        let arr = vec![2, 2, 3];
        assert_eq!(super::Solution::single_number(arr), 3)
    }
    #[test]
    fn single_num_ex2() {
        let arr = vec![4, 1, 2, 1, 2];
        assert_eq!(super::Solution::single_number(arr), 4)
    }
}
