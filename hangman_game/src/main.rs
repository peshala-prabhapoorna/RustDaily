extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io;
use std::io::prelude::*;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

fn main() {
    let mut attempts_left = ALLOWED_ATTEMPTS;
    let random_word = select_random_word();
    let mut letters = create_letters(&random_word);

    println!("Welcome to hangman!");

    loop {
        println!("\nYou have {} attempts left", attempts_left);
        display_progress(&letters);

        println!("\nPlease enter a letter to guess:");
        let user_input_char = read_user_input_char();

        //exit if a user enters an asterisk '*'
        if user_input_char == '*' {
            break;
        }

        /* update the 'revealed' state of each letter. If the user has guessed
        a correct letter, at least one revealed is changed to true */
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_input_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        //if guessed incorreclty lose a turn
        if at_least_one_revealed == false {
            attempts_left -= 1;
        }

        match check_progress(attempts_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongratulations, you won! The word was {}.", random_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nSorry, you lost! The word was {}.", random_word);
                break;
            }
        }
    }
    println!("\nGoodbye!");
}

fn select_random_word() -> String {
    //open file
    let mut file = File::open("words.txt").expect("Could not open file!");

    //load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error occured while reading the file!");

    //split file contents and store in a vec
    let avalable_words: Vec<&str> = file_contents.trim().split(',').collect();

    //generate random index
    let random_index = rand::thread_rng().gen_range(0, avalable_words.len());

    String::from(avalable_words[random_index])
}

fn create_letters(word: &String) -> Vec<Letter> {
    //create empty vector
    let mut letters: Vec<Letter> = Vec::new();

    //wrap each character in a Letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }

    letters
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); //eg. _ A _ M _ _

    //display letter or _ for each letter
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_char() -> char {
    let mut user_input = String::new();

    //get user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}

fn check_progress(attempts_left: u8, letters: &Vec<Letter>) -> GameProgress {
    //determine if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    //if you have attempts left and all letters have not been revealed
    if attempts_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
