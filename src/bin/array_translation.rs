pub fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6];
    trans(&mut arr, 4);
    println!("{:?}", arr);
}

fn trans(arr: &mut [u16], k: usize) {
    let n = arr.len();
    let count = gcd(n, k);
    let step = k % n;
    for i in 0..count {
        let mut current_index = i;
        let mut prev = arr[i];
        loop {
            let next = (current_index + step) % n;

            let t = arr[next];
            arr[next] = prev;
            prev = t;

            current_index = next;
            if current_index == i {
                break;
            }
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}
