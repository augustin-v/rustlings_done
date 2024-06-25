fn tribonacci_reccursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        3 => 2,
        _ => tribonacci_reccursive(n - 1) + tribonacci_reccursive(n - 2) + tribonacci_reccursive(n - 3),
    }
}

fn main() {
    println!("{}",tribonacci_reccursive(25));
}
