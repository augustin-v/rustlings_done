fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    let indexed_multiplication: Vec<i32> = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| x * i as i32)
        .collect();
    println!("{:?}",indexed_multiplication);
}