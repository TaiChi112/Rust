pub fn max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    max
}
