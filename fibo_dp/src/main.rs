fn fibonacci(n: i32) -> i32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
} 

fn main() {
    let a = fibonacci(5);
    println!("{a}");
    println!("gcd is: {}", gcd(192, 78));
}


fn gcd(a: u64, b: u64) -> u64{

    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let (_quot, rem) = div_rem(a, b);
    gcd(b, rem)
    
}

fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem =  x % y;
    (quot, rem)
}