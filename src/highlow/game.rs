use rand;
use std::io;
use colored::*;

pub struct Game {
    rounds: u8,
    current_number: u8,
}

pub enum RoundResult {
    Win,
    Tie,
    Lose,
}

impl Game {
    // Called on a Game struct instance
    // Returns a bool --> true = game finished, false = game still going
    pub fn play_game_round(&mut self) -> RoundResult {
        // Print game round header
        println!("\n************************************\nCurrent number: {} and current round: {}\n",
            self.current_number.to_string().blue(),
            self.rounds.to_string().blue());
        println!("Will the next number be higher? ({}/{})", "true".green(), "false".red());

        // Get users guess from CLI (bool)
        let guess = get_user_guess();

        // Get a random u8 integer 0 - 255 as the next number
        let next_number = rand::random::<u8>();

        // Display the next number
        println!("The next number is: {}", next_number.to_string().blue());

        // Display whether or not the user guessed successfully and return bool (state of the game)
        if guess && self.current_number < next_number || !guess && self.current_number > next_number {
            println!("You were {}!\n************************************\n",
                "correct".green());

            // Update Game struct (game state)
            self.current_number = next_number;
            self.rounds = self.rounds + 1;
            RoundResult::Win
        } else if self.current_number == next_number {
            println!("Tie {}!\n************************************\n",
                "correct".green());

            // Update Game struct (game state)
            // Tie is a no loss, no win, so round is not incremented
            self.current_number = next_number;
            RoundResult::Tie
        } else {
            println!("You were {}! You lasted {} round(s).", "wrong".red(), self.rounds.to_string().blue());
            RoundResult::Lose
        }
    }
}

// Creates and returns a new Game struct instance (technically a move???)
pub fn start_game() -> Game {
    Game {
        rounds: 0,
        current_number: rand::random::<u8>(),
    }
}

// Get CLI input from the user
// Returns bool --> true = user guessed higher
fn get_user_guess() -> bool {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<bool>() {
        Ok(i) => {
            return i;
        },
        Err(..) => {
            println!("Incorrectly formatted guess, try again.");
            return get_user_guess();
        },
    };
}
