use evalexpr::eval;
use std::io::*;

fn main() {
    let mut function_input = String::new();
    println!("Enter a function to find its root:");
    stdin()
        .read_line(&mut function_input)
        .expect("Failed to read line");
    let function_str: &str = function_input.trim();
    
    let mut a_input = String::new();
    println!("Enter a value for a:");
    stdin()
        .read_line(&mut a_input)
        .expect("Failed to read line");
    let a: f64 = a_input.trim().parse().expect("Please type a number!");

    let mut b_input = String::new();
    println!("Enter a value for b:");
    stdin()
        .read_line(&mut b_input)
        .expect("Failed to read line");
    let b: f64 = b_input.trim().parse().expect("Please type a number!");

    let mut tolerance_input = String::new();
    println!("Enter a value for tolerance:");
    stdin()
        .read_line(&mut tolerance_input)
        .expect("Failed to read line");
    let tolerance: f64 = tolerance_input.trim().parse().expect("Please type a number!");

    println!("root: {}", bisection(&function_str, a, b, tolerance));
}

fn bisection(function_str: &str, mut a: f64, mut b: f64, tolerance: f64) -> f64 {
    let mut c: f64 = 0.0;
    while (b - a) > tolerance {
        c = (b + a) / 2.0;
        if evaluate_function(function_str.to_owned(), c) > 0.0 {
            b = c;
        } else {
            a = c;
        }
    }
    c
}

fn evaluate_function(function_str: String, c: f64) -> f64 {
    let function_c_str: String = function_str.replace("x", &c.to_string());
    let function: evalexpr::Value = eval(&function_c_str).unwrap();
    function.as_float().unwrap()
}
