pub struct Solution;

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n > 0 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[test]
fn is_power3_ex1() {
    let x = 27;
    assert_eq!(Solution::is_power_of_three(x), true);
}

#[test]
fn is_power3_ex2() {
    let x = 0;
    assert_eq!(Solution::is_power_of_three(x), false);
}

#[test]
fn is_power3_ex3() {
    let x = 9;
    assert_eq!(Solution::is_power_of_three(x), true);
}

#[test]
fn is_power3_ex4() {
    let x = 45;
    assert_eq!(Solution::is_power_of_three(x), false);
}
