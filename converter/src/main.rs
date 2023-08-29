use std::io;

fn main() {
    let mut celcius_string: String  = String::new();
    println!("Enter temperature in degrees celcius (e.g. 25.5):");
    io::stdin().read_line(&mut celcius_string).expect("Input read error");
    let temp_celcius: f32 = celcius_string.trim().parse().expect("Failed to parse temperature as f32");
    let temp_fahrenheit: f32 = temp_celcius * 9f32 / 5f32 + 32f32;
    println!("Temperature in degrees fahrenheit: {temp_fahrenheit}");
}
