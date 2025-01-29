fn fibonacci(n: i32) -> i32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
} 

fn main() {
    let a = fibonacci(5);
    println!("{a}");
    println!("gcd is: {}", gcd(66528, 52920));
    println!("gcd is: {}, {} {}, ", extended_gcd(26513, 32321).0, extended_gcd(26513, 32321).1, extended_gcd(26513, 32321).2);
    println!("congruence: {}", find_congruence(8146798528947, 17));
}



// euclidean
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

// extended euclidean ```a * x + b * y = gcd(a,b)```
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a/b)*y)
    }
}

fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem =  x % y;
    (quot, rem)
}

// 11 ≡ x mod 6. find x
fn find_congruence(a: i64, b: i64) -> i64 {
    a % b //  = x
}