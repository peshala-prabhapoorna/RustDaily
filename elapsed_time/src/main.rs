mod factorial;

use factorial::factorial;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    println!("Enter a positive integer below 21:");
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();
    let now = Instant::now();

    println!("The factorial of {} is {}", n, factorial(n));

    let elapsed = now.elapsed();

    println!("Elapsed time: {} microseconds", elapsed.as_micros());
}
