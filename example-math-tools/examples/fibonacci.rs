use example_math_tools::addi;

/// Calculate the fibonacci number series for 5, 42 and 101
fn main() {
    println!("Fibonacci series of 5   is: {:?}",fibonacci(5));
    println!("Fibonacci series of 42  is: {:?}",fibonacci(42));
    println!("Fibonacci series of 101 is: {:?}",fibonacci(101));
}

fn fibonacci(x: i64) -> i64 {
    let mut f0 = 0i64;
    let mut f1 = 1i64;

    for _ in 2..=x {
        let f2 = addi(f0, f1).0;
        f0 = f1;
        f1 = f2;
    }

    f1
}
