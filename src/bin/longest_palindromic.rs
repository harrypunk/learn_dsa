pub fn main() {
    let str1 = "badad";
    let res = longest_palindrome(str1.to_owned());
    println!("{}", res)
}

fn longest_palindrome(s: String) -> String {
    let aug = add_holders(s.clone());
    let mut val_arr: Vec<usize> = Vec::with_capacity(aug.len());

    let mut r_max = 0usize;
    let mut id = 0usize;
    let mut i = 1usize;
    while i < aug.len() && r_max < aug.len() {
        if i >= id {
            let right1 = find_pal_right(&s, i);
            let radius1 = right1 - i;
            val_arr[i] = right1;
            if radius1 > (r_max - id) {
                r_max = right1;
                id = i;
            }
        } else {
        }

        i += 1;
    }

    let left = id - (r_max - id);
    let right = r_max;
    aug[left..right].to_string()
}

fn add_holders(s: String) -> String {
    let mut s1 = String::from('#');
    for c in s.chars() {
        s1.push(c);
        s1.push('#');
    }
    s1
}

fn find_pal_right(s: &str, index: usize) -> usize {
    let bytes = s.as_bytes();
    let mut left = index;
    let mut right = index;
    let mut right_max = right;
    while right < s.len() {
        if bytes[left] == bytes[right] {
            right_max = right;
            if left > 0 {
                left -= 1;
                right += 1;
            }
        }
    }
    return right_max;
}
