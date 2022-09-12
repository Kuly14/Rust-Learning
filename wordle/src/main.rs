mod word_list;
use rand::Rng;
use std::io;

// Green -> Correct place
// Blue -> Correct char wrong place
// Default -> Char not in word

fn main() {
    let accepted_words = word_list::uncommon();
    let words = word_list::word_list();
    let random = rand::thread_rng().gen_range(0..=words.len());
    let todays_word = words[random];
    println!("Guess is {}", todays_word);
    let mut count = 1;
    while count < 7 {
        println!("\nGues number {}", count);
        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");

        if guess.trim().len() != 5 {
            println!("Needs to be 5 character word");
            continue;
        }

        count += 1;

        wordle::check_if_won(&guess, todays_word);

        let first: Vec<_> = accepted_words
            .iter()
            .filter(|word| word.trim() == guess.trim())
            .collect();

        let second: Vec<_> = words
            .iter()
            .filter(|word| word.trim() == guess.trim())
            .collect();

        if first.len() == 0 && second.len() == 0 {
            println!("That's not a real word");
            println!("Try again");
            count -= 1;
            continue;
        }

        let todays_word_chars: Vec<char> = todays_word.clone().chars().into_iter().collect();

        wordle::check_if_correct(&guess, &todays_word_chars);
    }

    println!("\nGame Over");
}
