pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut left: usize = 0;
        let mut right: usize = 1;

        while right < nums.len() {
            let r = nums[right];
            if r > nums[left] {
                nums[left + 1] = r;
                left += 1;
            }

            right += 1;
        }

        (left + 1) as i32
    }
}

mod tests {
    #[test]
    fn rmdup_ex1() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let k = super::Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 2);
    }

    #[test]
    fn rmdup_ex2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = super::Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 5);

        let expected = vec![0, 1, 2, 3, 4];
        for i in 0..k {
            assert_eq!(nums[i], expected[i]);
        }
    }
    #[test]
    fn rmdup_ex3() {
        let mut nums: Vec<i32> = vec![1, 2];
        let k = super::Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 2);

        let expected = vec![1, 2];
        for i in 0..k {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
