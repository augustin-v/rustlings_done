fn fibonacci(n: i32) -> i32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
} 

fn main() {
    let a = fibonacci(5);
    println!("{a}");
}
