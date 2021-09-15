//use std::collections::HashMap;

pub fn main() {
    let arr1 = vec![6, 5, 1, 3, 2, 7, 11, 9, 23];

    let result = get_lm(&arr1);
    println!("{:?}", result);
}

fn get_lm(arr: &[u16]) -> usize {
    let mut dp = vec![1; arr.len()];
    let mut maxans = 1u16;
    for i in 1..arr.len() {
        dp[i] = 1;
        for j in 0..i {
            if arr[i] > arr[j] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
        maxans = std::cmp::max(maxans, dp[i]);
        //println!("dp {:?},", dp);
    }
    maxans as usize
}
