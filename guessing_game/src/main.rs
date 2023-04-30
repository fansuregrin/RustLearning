use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        print_main_menu();

        let mut choice_buf = String::new();
        loop {
            choice_buf.clear();
            io::stdin()
                .read_line(&mut choice_buf)
                .expect("Failed to read line");
            let choice: u8;
        
            choice = match choice_buf.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    print!("Please input a choice integer on the menu: ");
                    io::stdout().flush().unwrap();
                    continue;
                },
            };
            match choice {
                1 => {
                    start_game();
                    break;
                }
                2 => return,
                _ => {
                    print!("Please input a choice integer on the menu: ");
                    io::stdout().flush().unwrap();
                    continue;
                }
            }
        }
    }
}

fn print_main_menu() {
    println!("+==========================================+");
    println!("+            Number Guessing Game          +");
    println!("+------------------------------------------+");
    println!("+  1: Start the game                       +");
    println!("+  2: Quit                                 +");
    println!("+------------------------------------------+");
    print!("+ Please input your choice: ");
    io::stdout().flush().unwrap();
}

fn start_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}