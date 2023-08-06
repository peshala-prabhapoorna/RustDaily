use std::io::{self};

fn main() {
    let mut water_level = 3; // Half-filled jug at the start

    loop {
        print_jug(water_level);
        println!("Enter 'fill' to add water, 'spill' to remove water, or 'quit' to exit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let action = input.trim().to_lowercase();

        match action.as_str() {
            "fill" => water_level = fill_jug(water_level),
            "spill" => water_level = spill_jug(water_level),
            "quit" => {
                println!("Exiting the game.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 'fill', 'spill', or 'quit'.");
            }
        }
    }
}

fn print_jug(water_level: i32) {
    let total_capacity = 6;
    let remaining_capacity = total_capacity - water_level;

    for i in 1..=total_capacity {
        if i <= remaining_capacity {
            println!("|       |");
        } else {
            println!("|~~~~~~~|");
        }
    }
    println!("`-------`");
}

fn fill_jug(current_level: i32) -> i32 {
    if current_level < 6 {
        println!("Filling the jug...");
        current_level + 1
    } else {
        println!("The jug is already full.");
        current_level
    }
}

fn spill_jug(current_level: i32) -> i32 {
    if current_level > 0 {
        println!("Spilling the jug...");
        current_level - 1
    } else {
        println!("The jug is already empty.");
        current_level
    }
}
