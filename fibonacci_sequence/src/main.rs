use std::io;

fn main() {
    let mut n_string: String = String::new();
    println!("Enter the term number of fibonacci sequence:");
    io::stdin()
        .read_line(&mut n_string)
        .expect("failed to read line");
    let n: u16 = n_string.trim().parse().expect("failed to parse to integer");

    let mut term_n_2: u16 = 0;
    let mut term_n_1: u16 = 1;

    for i in 2..n + 1  {
        let term_n: u16 = term_n_2 + term_n_1;
        term_n_2 = term_n_1;
        term_n_1 = term_n;
        if i == n {
            println!("{n}th term of the fibonacci sequence is {term_n}.")
        }
    }
}
