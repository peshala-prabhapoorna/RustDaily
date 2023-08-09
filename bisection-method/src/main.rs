use std::io::*;
use evalexpr::eval;

fn main() {
    let mut user_input = String::new();
    println!("Enter a function to find its root:");
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("{}", bisection(&user_input, 0.0, 2.0, 0.0001));
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
    let function_c_str: String  = function_str.replace("x", &c.to_string());
    let function: evalexpr::Value = eval(&function_c_str).unwrap();
    function.as_float().unwrap()
}
