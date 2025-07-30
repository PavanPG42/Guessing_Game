use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🎲 Welcome to the Guess the Number Game!");
    println!("🔢 I'm thinking of a number between 1 and 100...");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("📝 Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        println!("👉 You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small!"),
            Ordering::Greater => println!("📈 Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;
            }
        }
    }
// End of the game
    println!("👋 Thanks for playing! Goodbye!");
}