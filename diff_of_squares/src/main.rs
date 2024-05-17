
// This program calculates the squared sum of a given number, the sum of squares of numbers up to the given number, and the difference between these two values.

fn main() {
    // Initialize the variable n with the value 10
    let n = 10;

    // Print the results of the calculations using the functions below
    println!(
    "\n     n is {n}, it's squared sum is {}, it's sum squared is {},\n
        and the difference between the two is {}!",
        sq_sum(n),
        sum_sq(n),
        diff_sq(n)
    );
}

/// Calculates the difference between the squared sum and the sum of squares up to n
fn diff_sq(n: usize) -> usize {
    sq_sum(n) - sum_sq(n)
}

/// Calculates the squared sum of numbers up to n
fn sq_sum(mut n: usize) -> usize {
    
    if n < 1 {
        panic!("The number should be positive.")
    }   else if n < 2 {
            return 1;
        }

    // Create a vector to store numbers up to n,
    // that can contain up to n values.
    let mut a: Vec<usize> = Vec::with_capacity(n);
    while n > 0 {
        a.push(n);
        n -= 1;
    }

    // Calculate the squared sum
    let b = a.iter().fold(0, |x, y| x + y).pow(2);
    b
}

/// Calculates the sum of squares of numbers up to n
fn sum_sq(mut n: usize) -> usize {
    if n < 2 {
        return 1;
    }

    // Create a vector to store squared numbers up to n
    let mut a: Vec<usize> = Vec::with_capacity(n);
    while n > 0 {
        a.push(n.pow(2));
        n -= 1;
    }

    // Calculate the sum of squares
    let b = a.iter().fold(0, |x, y| x + y);
    b
}

