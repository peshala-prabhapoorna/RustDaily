fn main() {
    println!("{}", bisection(|x| x * x * x - 2.0, 0.0, 2.0, 0.0001));
}

fn bisection(function: fn(f64) -> f64, mut a: f64, mut b: f64, tolerance: f64) -> f64 {
    let mut c: f64 = 0.0;
    while (b - a ) > tolerance {
        c = (b + a) / 2.0;
        if function(c) > 0.0{
            b = c;
        } else {
            a = c;
        }        
    }
    c
}
