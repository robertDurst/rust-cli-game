use rand;
use std::io;
use colored::*;

pub struct Game {
    rounds: u8,
    current_number: u8,
}

impl Game {
    pub fn play_game_round(&mut self) -> bool {
        println!("\n************************************", );
        println!("Current number: {} and current round: {}\n",
            self.current_number.to_string().blue(),
            self.rounds.to_string().blue());
        println!("Will the next number be higher? ({}/{})", "true".green(), "false".red());
        let guess = get_user_guess();
        let next_number = rand::random::<u8>();
        println!("The next number is: {}", next_number.to_string().blue());
        if guess && self.current_number < next_number || !guess && self.current_number >= next_number {
            println!("You were {}!", "correct".green());
            println!("************************************\n", );
            self.current_number = next_number;
            self.rounds = self.rounds + 1;
            false
        } else {
            println!("You were {}! You lasted {} round(s).", "wrong".red(), self.rounds.to_string().blue());
            true
        }
    }
}

pub fn start_game() -> Game {
    Game {
        rounds: 0,
        current_number: rand::random::<u8>(),
    }
}

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
