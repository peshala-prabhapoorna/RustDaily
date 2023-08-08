use std::str::FromStr;
use std::ffi::CStr;
use std::io;

trait MyFunction {
    fn apply(self, x: f64) -> f64;
}

impl Sized for dyn MyFunction {}

impl MyFunction for fn(f64) -> f64 {
    fn apply(self, x: f64) -> f64 {
        self(x)
    }
}

impl FromStr for dyn MyFunction {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let function_str = s.trim();
        if function_str.is_empty() {
            return Err(std::string::ParseError::Empty);
        }

        let function: fn(f64) -> f64 = unsafe {
            let c_str = CStr::new(function_str).unwrap();
            let c_function = std::ffi::CString::new(c_str.to_bytes()).unwrap();
            std::mem::transmute(c_function)
        };

        Ok(function)
    }
}

fn main() {
    let mut a: f64;
    let mut b: f64;
    let mut tolerance: f64;

    println!("Enter the function:");
    let function_str = read_line().unwrap().trim();
    let function: MyFunction = function_str.parse().unwrap();

    println!("Enter the lower bound:");
    a = read_line().parse().unwrap();

    println!("Enter the upper bound:");
    b = read_line().parse().unwrap();

    println!("Enter the tolerance:");
    tolerance = read_line().parse().unwrap();

    println!("{}", bisection(function, a, b, tolerance));
}

fn bisection(function: dyn MyFunction, mut a: f64, mut b: f64, tolerance: f64) -> f64 {
    let mut c: f64 = 0.0;
    while (b - a ) > tolerance {
        c = (b + a) / 2.0;
        if function.apply(c) > 0.0{
            b = c;
        } else {
            a = c;
        }        
    }
    c
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
