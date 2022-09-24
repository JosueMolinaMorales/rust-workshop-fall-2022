use std::{ io::{self, Write}, cmp::Ordering };
use rand::prelude::*;
fn main() {
    let mut name_of_user = String::new();
    let mut number_of_games_played = 0;

    print!("Welcome the my guessing game! Please enter your name to begin >>> ");
    io::stdout().flush().unwrap(); // if print is used, need to flush out stdout
    io::stdin().read_line(&mut name_of_user).expect("There was an error reading input");

    // check to see if user is correct
    // if not correct, increment number_of_guesses
    // if correct, break out of inner loop, ask if they would like to play again
    // if play_again, increment number_of_games_played
    'main_game: loop {
        // increase number_of_games_played
        number_of_games_played += 1;
        // reset number_of guesses
        let mut number_of_guesses = 0;
        // Get the random number
        let random_number: u32 = rand::thread_rng().gen_range(1..=100);
        // enter loop, get users guess
        'guessing_loop: loop {
            let mut guess = String::new();
            print!("Please enter your guess >>> ");
            io::stdout().flush().unwrap(); // if print is used, need to flush out stdout
            io::stdin().read_line(&mut guess).expect("Error reading guess");
            let guess = match guess.trim_end().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number!");
                    continue;
                }
            };
            number_of_guesses += 1;
            match guess.cmp(&random_number) {
                Ordering::Less => println!("To small! Try again."),
                Ordering::Greater => println!("To big! Try again."),
                Ordering::Equal => {
                    println!("You guessed it in {number_of_guesses} guesses!");
                    break 'guessing_loop;
                }
            }
        }

        print!("Would you like to play again? (y/n) >>> ");
        io::stdout().flush().unwrap(); // if print is used, need to flush out stdout
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Error reading play again");
        match play_again.to_lowercase().chars().next().unwrap() {
            'y' => continue 'main_game,
            _ => break 'main_game
        }
    }

    println!("Thanks for playing {name_of_user}! You played {number_of_games_played} times!");
}
