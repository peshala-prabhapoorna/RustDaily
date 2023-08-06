use std::f64;
use std::io;

// function to calculate distance between two points
fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x_diff: f64 = p2.0 - p1.0;
    let y_diff: f64 = p2.1 - p1.1;
    let distance: f64 = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
    distance
}
fn main() {
    // Take input from the user for point1
    println!("Enter the coordinates of point1 (separated by a space):");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut coordinates = input.split_whitespace();
    let x1: f64 = coordinates
        .next()
        .expect("Invalid input for x1")
        .parse()
        .expect("Invalid input for x1");
    let y1: f64 = coordinates
        .next()
        .expect("Invalid input for y1")
        .parse()
        .expect("Invalid input for y1");

    // Take input from the user for point2
    println!("Enter the coordinates of point2 (separated by a space):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut coordinates = input.split_whitespace();
    let x2: f64 = coordinates
        .next()
        .expect("Invalid input for x2")
        .parse()
        .expect("Invalid input for x2");
    let y2: f64 = coordinates
        .next()
        .expect("Invalid input for y2")
        .parse()
        .expect("Invalid input for y2");

    // Create tuples with the user inputs
    let point1 = (x1, y1);
    let point2 = (x2, y2);

    let distance: f64 = euclidean_distance(point1, point2);

    println!(
        "The Eudlidean distance between {:?} and {:?} is {}",
        point1, point2, distance
    )
}
