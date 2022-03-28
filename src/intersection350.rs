use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Solution::intersect(nums2, nums1);
        }
        let mut elements: HashMap<i32, u16> = HashMap::new();
        let mut result = Vec::new();

        for v in nums1 {
            if let Some(count) = elements.get_mut(&v) {
                *count += 1;
            } else {
                elements.insert(v, 1);
            }
        }
        for v in nums2 {
            if let Some(count) = elements.get_mut(&v) {
                if *count > 0 {
                    result.push(v);
                    *count -= 1;
                }
            }
        }

        result
    }
}

#[test]
fn intersection_ex1() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let result = Solution::intersect(nums1, nums2);
    assert_eq!(result, [2, 2]);
}

#[test]
fn intersection_ex2() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let result = Solution::intersect(nums1, nums2);
    assert_eq!(result, [4, 9]);
}
