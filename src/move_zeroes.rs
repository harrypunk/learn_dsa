pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for i in 1..nums.len() {
            if nums[i] != 0 && nums[left] == 0 {
                nums[left] = nums[i];
                nums[i] = 0;
            }
            if nums[left] != 0 {
                left += 1;
            }
        }
    }
}

mod tests {
    #[test]
    fn move_zero_ex1() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        super::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn move_zero_ex2() {
        let mut nums: Vec<i32> = vec![1, 0, 1];
        super::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 1, 0]);
    }
    #[test]
    fn move_zero_ex3() {
        let mut nums: Vec<i32> = vec![4, 2, 4, 0, 0, 3, 0, 5, 1, 0];
        super::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![4, 2, 4, 3, 5, 1, 0, 0, 0, 0]);
    }
}
