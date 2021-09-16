use flexi_logger::{Logger, WriteMode};

pub fn main() {
    let _logger = Logger::try_with_env_or_str("info")
        .expect("logger env failed")
        .write_mode(WriteMode::Async)
        .log_to_stdout()
        .start()
        .expect("logger start error");

    let mut arr = vec![2, 3, 3, 3, 5, 6, 7, 7, 7, 8, 8, 9, 9];
    let last = deduplicate(&mut arr);
    log::info!("last {:?}", last);
}

//deduplicate(mutate) an sorted array, return length
fn deduplicate(arr: &mut [u16]) -> usize {
    log::debug!("init :{:?}", arr);
    let mut left_index: usize = 0;
    let len = arr.len();
    for right_index in 1..len {
        log::debug!("curr :{:?}, L:{}, R:{}", arr, left_index, right_index);
        if arr[right_index] > arr[right_index - 1] {
            arr[left_index + 1] = arr[right_index];
            left_index += 1;
        }
    }

    left_index + 1
}
