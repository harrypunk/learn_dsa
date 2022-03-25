use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }

    pub fn single_number2(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, bool> = HashMap::new();
        for x in nums {
            if map.get(&x).is_none() {
                map.insert(x, true);
            } else {
                map.remove(&x);
            }
        }
        map.keys().fold(0, |_, x| *x)
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
    #[test]
    fn single_num_ex3() {
        let arr = vec![2, 2, 3];
        assert_eq!(super::Solution::single_number2(arr), 3)
    }
    #[test]
    fn single_num_ex4() {
        let arr = vec![4, 1, 2, 1, 2];
        assert_eq!(super::Solution::single_number2(arr), 4)
    }
}
