use std::process;
use colored::Colorize;

pub fn check_if_won(guess: &String, todays_word: &str) {
    if guess.trim() == todays_word.trim() {
        println!("{}", todays_word.green());
        println!("You won!!!");
        process::exit(1);
    }
}

pub fn check_if_correct(guess: &String, todays_word_chars: &Vec<char>) {
    for (index, character) in guess.trim().chars().enumerate() {
        if character == todays_word_chars[index] {
            print!("{}", &character.to_string().green());
        } else {
            if todays_word_chars.contains(&character) {
                print!("{}", &character.to_string().blue());
            } else {
                print!("{}", &character);
            }
        }
    }
}

