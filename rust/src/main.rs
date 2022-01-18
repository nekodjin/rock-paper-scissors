mod rps;
mod end_state;

pub use rps::RPS;
pub use end_state::EndState;

use std::io::stdin;

macro_rules! print_flush {
    ($e:tt) => {
        print!($e);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
}

fn main() {
    use EndState::*;

    println!("Welcome to Rock, Paper, Scissors!");
    
    loop {
        println!();
        print_flush!("Enter your choice: ");

        let comp = rand::random();
        let user = user_choice();

        let result = EndState::from(user, comp);

        println!("You chose: {:?}", user);
        println!("Computer chose: {:?}", comp);
        
        match result {
            Win  => println!("You won!"),
            Loss => println!("You lost :("),
            Tie  => println!("You tied."),
        }

        print_flush!("Play again? (y/N) ");

        if "y" == read_line().as_str() {
            continue;
        }

        break;
    }

    println!("Thanks for playing!");
}

fn user_choice() -> RPS {
    loop {
        let t = read_line().parse();
        
        if t.is_ok() {
            return t.unwrap();
        }

        print_flush!("Invalid input. Please try again: ");
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

