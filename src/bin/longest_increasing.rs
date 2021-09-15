//use std::collections::HashMap;

pub fn main() {
    let arr1 = vec![6, 5, 1, 3, 2, 7, 11, 9, 23];

    let result = get_lm(&arr1);
    println!("{:?}", result);
}

fn get_lm(arr: &[u16]) -> Vec<u16> {
    let mut list = Vec::with_capacity(0);
    for i in 0..arr.len() {
        let max_sub = calc_max(arr, i);
        if max_sub.len() > list.len() {
            list = max_sub;
        }
    }
    list
}

fn calc_max(arr: &[u16], start_index: usize) -> Vec<u16> {
    let mut list = Vec::new();
    list.push(arr[start_index]);
    for i in start_index..arr.len() {
        let next = arr[i];
        if next > list[list.len() - 1] {
            list.push(next);
        }
    }

    list
}
