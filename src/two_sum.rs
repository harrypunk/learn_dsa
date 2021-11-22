use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (index, &value) in nums.iter().enumerate() {
        if let Some(&i) = map.get(&(target - value)) {
            return vec![i as i32, index as i32];
        }
        map.insert(value, index);
    }

    panic!()
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

#[test]
#[should_panic]
fn two_sum_exm4() {
    let n1 = vec![3, 3, 1, 12];
    let _ = two_sum(n1, 7);
}
