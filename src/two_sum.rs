use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = 0;

    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let current = nums[i];
        if let Some(j) = map.get(&current) {
            left = *j;
            right = i;
            break;
        } else {
            let diff = target - current;
            map.insert(diff, i);
        }
    }

    vec![left as i32, right as i32]
}

#[test]
fn two_sum_exm1() {
    let n1 = vec![2, 7, 11, 15];
    let res = two_sum(n1, 9);
    assert_eq!(res, [0, 1]);
}

#[test]
fn two_sum_exm2() {
    let n1 = vec![3, 2, 4];
    let res = two_sum(n1, 6);
    assert_eq!(res, [1, 2]);
}

#[test]
fn two_sum_exm3() {
    let n1 = vec![3, 3];
    let res = two_sum(n1, 6);
    assert_eq!(res, [0, 1]);
}
