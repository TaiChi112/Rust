

fn main() {
    let mut amount:[i8;256] = [0;256];

    for i in 0..256 {
        amount[i] = i as i8;
    }
    fn array_length<T>(array: &[T]) -> usize {
        let mut length = 0;
        for _ in array {
            length += 1;
        }
        length
    }
    println!("Length of the array is: {}", array_length(&amount));
}
